use std::fmt::Display;


fn some_work(name: String) {
    println!("{name}");
}

fn some_work_2(age: u8) {
    println!("{age}");
} 

fn main() {
    let code = 403;
    let response = request(code);
    match response {
        Response::ItsOk => println!("It's ok"),
        Response::ItsNotOk => println!("It's not ok"),
        Response::ItsNotOkWithReason(reason) => println!("It's not ok with reason: {reason}"),
        Response::ItsNotOkWithReasonAndCode(reason, code) => println!("It's not ok with reason: {reason} and code: {code}"),
    }
    let statuses: Vec<Response> = vec![Response::ItsOk, Response::ItsNotOk, Response::ItsNotOkWithReason("Unauthorized".to_string()), Response::ItsNotOkWithReasonAndCode("Forbidden".to_string(), 403)];
    
    let name: String = "Dima".to_string();
    // name - no Copy (move the value)
    let name_2: &str = "Max";
    // name - Copy (copy the value)
    some_work(name.clone());
    println!("Hello, {name_2}");
    println!("Hello, {name}");

    let age: u8 = 30;
    some_work_2(age);
    println!("Age: {age}");


    let pers_1 = Person {
        name: "Dima".to_string(),
        age: 30,
        is_student: false,
    };

    let pers_2 = create_person("Yura".to_string(), 30);

    println!("src person: {pers_1}");
    let new_student = teleport_person(pers_2);

    let pers_3 = Person::new("Alex".to_string(), 25);

    println!("new student: {new_student}");
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
        Person {
            name,
            age,
            is_student: false,
        }
    }
}

fn teleport_person(person: Person) -> Person {
    let new_pers = Person {
        name: person.name,
        age: person.age,
        is_student: true,
    };
    new_pers
}

enum Response {
    ItsOk,
    ItsNotOk,
    ItsNotOkWithReason(String),
    ItsNotOkWithReasonAndCode(String, u16),
}

fn request(code: u16) -> Response {
    if code == 200 {
        Response::ItsOk
    } else if code == 400 {
        Response::ItsNotOk
    } else if code == 401 {
        Response::ItsNotOkWithReason("Unauthorized".to_string())
    } else if code == 403 {
        Response::ItsNotOkWithReasonAndCode("Forbidden".to_string(), code)
    } else {
        Response::ItsNotOkWithReason("Unknown".to_string())
    }
}
