use clap::Parser;
use utility_crates::data::{Args, Company, MyStruct, Person, parse_request_line};

fn main() {
    let value = MyStruct { a: 42, b: "hello" };
    let json = serde_json::to_string(&value).unwrap();
    println!("JSON: {json}");

    let company = Company(vec![Person { id: 1, age: 24 }, Person { id: 2, age: 42 }]);
    let config = bincode::config::standard();
    let encoded = bincode::encode_to_vec(&company, config).unwrap();
    let (decoded, _): (Company, usize) = bincode::decode_from_slice(&encoded, config).unwrap();
    println!(
        "bincode: {} байт, восстановлено: {decoded:?}",
        encoded.len()
    );

    let (_, request) = parse_request_line("GET /path/get_example HTTP/1.1 200").unwrap();
    println!("nom: {request:?}");

    let args = Args::try_parse_from(["utility-demo", "--string-arg", "hello"]).unwrap();
    println!("clap: {args:?}");
}
