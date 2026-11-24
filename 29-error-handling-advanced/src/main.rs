use errors_advanced::errors::{Error, SeverityLevel, TransactionError};

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

#[derive(Default)]
struct Service {
    users: Vec<User>,
}

impl Service {
    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    fn get_user_mut(&mut self, id: u32) -> Result<&mut User, Error> {
        if id == 0 {
            return Err(Error::InvalidInput {
                field: "id".to_string(),
            });
        }
        self.users
            .iter_mut()
            .find(|user| user.id == id)
            .ok_or(Error::NotFound)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        eprintln!("{:?}: {info}", SeverityLevel::Error);
        previous_hook(info);
    }));

    let mut service = Service::default();
    service.add_user(User {
        id: 1,
        name: "John".to_string(),
    });
    let user = service.get_user_mut(1)?;
    println!("User {}: {}", user.id, user.name);

    let transaction_error = Error::from(TransactionError::InsufficientFunds);
    println!(
        "{transaction_error}; retryable = {}, severity = {:?}",
        transaction_error.retryable(),
        transaction_error.severity_level()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_expose_retry_and_severity_contracts() {
        let input = Error::InvalidInput {
            field: "id".to_string(),
        };
        assert!(!input.retryable());
        assert_eq!(input.severity_level(), SeverityLevel::Warning);
        assert!(Error::BadService.retryable());
        assert!(Error::Io(std::io::Error::other("тест")).retryable());

        let variants = [
            Error::AlreadyExists(1, "John".to_string()),
            Error::Custom("повторить".to_string()),
            Error::AnotherCustom("не повторять".to_string()),
            Error::from(TransactionError::InvalidInput),
        ];
        assert_eq!(variants[3].severity_level(), SeverityLevel::Info);
        assert_eq!(variants.len(), 4);
    }
}
