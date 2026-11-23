use std::fmt::Display;

fn some_work(name: String) {
    println!("{name}");
}

fn some_work_2(age: u8) {
    println!("{age}");
}

fn main() {
    let response = request(403);
    print_response(&response);

    let statuses = [
        Response::Ok,
        Response::NotOk,
        Response::NotOkWithReason("Unauthorized".to_string()),
        Response::NotOkWithReasonAndCode("Forbidden".to_string(), 403),
    ];
    for status in &statuses {
        print_response(status);
    }

    // String не реализует Copy: clone создаёт независимую копию для some_work.
    let name = "Dima".to_string();
    let name_2: &str = "Max";
    some_work(name.clone());
    println!("Hello, {name_2}");
    println!("Hello, {name}");

    // u8 реализует Copy, поэтому age доступен после вызова функции.
    let age: u8 = 30;
    some_work_2(age);
    println!("Age: {age}");

    let person = Person {
        name: "Dima".to_string(),
        age: 30,
        is_student: false,
    };
    let new_student = teleport_person(create_person("Yura".to_string(), 30));
    let another_person = Person::new("Alex".to_string(), 25);

    println!("src person: {person}");
    println!(
        "new student: {new_student}, student: {}",
        new_student.is_student
    );
    println!("another person: {another_person}");
}

fn print_response(response: &Response) {
    match response {
        Response::Ok => println!("It's ok"),
        Response::NotOk => println!("It's not ok"),
        Response::NotOkWithReason(reason) => println!("It's not ok with reason: {reason}"),
        Response::NotOkWithReasonAndCode(reason, code) => {
            println!("It's not ok with reason: {reason} and code: {code}")
        }
    }
}

fn create_person(name: String, age: u8) -> Person {
    Person {
        name,
        age,
        is_student: false,
    }
}

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u8,
    is_student: bool,
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is {} year old", self.name, self.age)
    }
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self {
            name,
            age,
            is_student: false,
        }
    }
}

fn teleport_person(person: Person) -> Person {
    Person {
        is_student: true,
        ..person
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Response {
    Ok,
    NotOk,
    NotOkWithReason(String),
    NotOkWithReasonAndCode(String, u16),
}

fn request(code: u16) -> Response {
    match code {
        200 => Response::Ok,
        400 => Response::NotOk,
        401 => Response::NotOkWithReason("Unauthorized".to_string()),
        403 => Response::NotOkWithReasonAndCode("Forbidden".to_string(), code),
        _ => Response::NotOkWithReason("Unknown".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::{Person, Response, create_person, request, teleport_person};

    #[test]
    fn maps_status_codes_to_responses() {
        assert_eq!(request(200), Response::Ok);
        assert_eq!(request(400), Response::NotOk);
        assert_eq!(
            request(403),
            Response::NotOkWithReasonAndCode("Forbidden".to_string(), 403)
        );
    }

    #[test]
    fn teleports_person_and_makes_them_a_student() {
        let person = teleport_person(create_person("Yura".to_string(), 30));

        assert_eq!(person.name, "Yura");
        assert_eq!(person.age, 30);
        assert!(person.is_student);
    }

    #[test]
    fn associated_constructor_creates_person() {
        let person = Person::new("Alex".to_string(), 25);

        assert_eq!(person.to_string(), "Alex is 25 year old");
        assert!(!person.is_student);
    }
}
