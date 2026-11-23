use std::collections::HashMap;

pub trait PaymentProcessor {
    fn process(&self, amount: f32) -> Result<(), String>;
}

pub struct CreditCardProcessor;
pub struct CryptoProcessor;
pub struct BankTransferProcessor;

impl PaymentProcessor for CreditCardProcessor {
    fn process(&self, amount: f32) -> Result<(), String> {
        self.log_payment(amount);
        Ok(())
    }
}

impl PaymentProcessor for CryptoProcessor {
    fn process(&self, amount: f32) -> Result<(), String> {
        self.log_payment(amount);
        Ok(())
    }
}

impl PaymentProcessor for BankTransferProcessor {
    fn process(&self, amount: f32) -> Result<(), String> {
        self.log_payment(amount);
        Ok(())
    }
}

pub trait Logger {
    fn log_payment(&self, amount: f32);
}

impl<T: PaymentProcessor> Logger for T {
    fn log_payment(&self, amount: f32) {
        println!("Payment processed: {amount}");
    }
}

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub struct Square {
    pub side: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

pub fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}

pub trait Workable {
    fn work(&self) -> &'static str;
}

pub trait Eatable {
    fn eat(&self) -> &'static str;
}

pub trait Sleepable {
    fn sleep(&self) -> &'static str;
}

pub struct Human;
pub struct Robot;

impl Workable for Human {
    fn work(&self) -> &'static str {
        "Human is working"
    }
}

impl Eatable for Human {
    fn eat(&self) -> &'static str {
        "Human is eating"
    }
}

impl Sleepable for Human {
    fn sleep(&self) -> &'static str {
        "Human is sleeping"
    }
}

impl Workable for Robot {
    fn work(&self) -> &'static str {
        "Robot is working"
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: u64,
    pub name: String,
}

pub struct RegisterData {
    pub id: u64,
    pub name: String,
}

impl From<RegisterData> for User {
    fn from(data: RegisterData) -> Self {
        Self {
            id: data.id,
            name: data.name,
        }
    }
}

pub trait UserRepository {
    fn find_by_id(&self, id: u64) -> Option<User>;
    fn save(&mut self, user: &User);
}

pub struct UserService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn register(&mut self, data: RegisterData) -> User {
        let user = User::from(data);
        self.repository.save(&user);
        user
    }

    pub fn find(&self, id: u64) -> Option<User> {
        self.repository.find_by_id(id)
    }
}

#[derive(Default)]
pub struct InMemoryUserRepository {
    storage: HashMap<u64, User>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl UserRepository for InMemoryUserRepository {
    fn find_by_id(&self, id: u64) -> Option<User> {
        self.storage.get(&id).cloned()
    }

    fn save(&mut self, user: &User) {
        self.storage.insert(user.id, user.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payment_processors_extend_common_trait() {
        let processors: Vec<&dyn PaymentProcessor> = vec![
            &CreditCardProcessor,
            &CryptoProcessor,
            &BankTransferProcessor,
        ];
        for processor in processors {
            assert_eq!(processor.process(100.0), Ok(()));
        }
    }

    #[test]
    fn shapes_are_substitutable() {
        let rectangle = Rectangle {
            width: 5.0,
            height: 3.0,
        };
        let square = Square { side: 10.0 };
        assert_eq!(rectangle.area(), 15.0);
        assert_eq!(square.area(), 100.0);
    }

    #[test]
    fn robot_only_implements_work() {
        assert_eq!(Robot.work(), "Robot is working");
        assert_eq!(Human.eat(), "Human is eating");
    }

    #[test]
    fn service_depends_on_repository_trait() {
        let mut service = UserService::new(InMemoryUserRepository::new());
        let user = service.register(RegisterData {
            id: 10,
            name: "user".into(),
        });
        assert_eq!(service.find(user.id), Some(user));
    }
}
