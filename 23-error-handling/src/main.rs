use std::{collections::HashMap, fmt};

#[derive(Debug)]
enum Error {
    DivisionByZero,
    NegativeNumber,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Error::DivisionByZero => "Division by zero error",
            Error::NegativeNumber => "Negative number error",
        };
        formatter.write_str(message)
    }
}

// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn do_some_work() -> Result<String, Error> {
    let x = -5;
    let y = 1;
    if y == 0 {
        return Err(Error::DivisionByZero);
    }
    if x < 0 {
        return Err(Error::NegativeNumber);
    }
    let z = x / y;
    Ok(format!("Result is {}", z))
}

fn get_user_id(user_name: &str) -> Option<String> {
    let mut base = HashMap::new();
    base.insert("user_1", "1");
    base.insert("user_2", "2");
    base.insert("user_3", "3");
    base.insert("admin", "4");
    if user_name != "admin" {
        let uid: &str = match base.get(user_name) {
            Some(id) => id,
            None => "unavailable",
        };
        return Some(uid.to_string());
    }
    None
}

fn main() {
    // match do_some_work() {
    //     Ok(result) => println!("{}", result),
    //     Err(error) => println!("{:?}", error),
    // };
    // println!("Result is {}", do_some_work().unwrap_or("Oh oh oh".to_string()));
    if let Ok(rrr) = do_some_work() {
        println!("Result is {}", rrr);
    }

    if let Some(id) = get_user_id("admin") {
        println!("User id is {}", id);
    }

    // let id = get_user_id("admin");
    println!("Hello, world!");
}
