mod errors;
use errors::Error;

use crate::errors::UserError;

fn do_something_good() -> Result<(), Error> {
    Ok(())
}

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    age: u8,
    email: String,
}

impl User {
    fn new(id: u32, name: String, age: u8, email: String) -> Self {
        Self {
            id,
            name,
            age,
            email,
        }
    }

}

struct Service {
    users: Vec<User>,
}
impl Service {
    fn new() -> Self {
        Self {
            users: vec![],
        }
    }

    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }
    
    fn get_user(&self, id: u32) -> Option<&User> {
        self.users.iter().find(|user| user.id == id)
    }
    
    fn get_user_mut(&mut self, id: u32) -> Result<&mut User, Error> {
        if id == 0 {
            return Err(Error::InvalidInput { field: "id".to_string() });
        }
        // if let Err(error) = db.init().connect() {
        //     Err(Error::IoError(error))
        // }
        match self.users.iter_mut().find(|user| user.id == id) {
            Some(user) => Ok(user),
            None => Err(Error::NotFound),
        }
    }
}

fn do_something_bad(num: i32) {
    let mut service = Service::new();
    let user = User::new(1, "John".to_string(), 30, "john@example.com".to_string());
    service.add_user(user);
    
    if num % 2 == 0 {
        panic!("Something bad happened");
    }
}

async fn do_something_async() -> Result<(), TransactionError> {
    service.do_something_async_first().await?;
    // VVVVVVVV <- cancelation()
    service.do_something_async_second().await?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|info| {
        hook(info);
        logger.log(SeverityLevel::Error, info);
        
    }));
    let mut service = Service::new();
    let user = User::new(1, "John".to_string(), 30, "john@example.com".to_string());
    service.add_user(user);
    let user = service.get_user(1)?;

    let thread = std::thread::Builder::new().name("worker_wallet_yield").spawn(|user| {*user.balance *= 1.01});

    println!("User: {:?}", user);
    Ok(())
}
