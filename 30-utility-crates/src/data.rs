use bincode::{Decode, Encode};
use clap::Parser;
use nom::IResult;
use nom::Parser as _;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{digit1, space1};
use nom::combinator::map_res;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MyStruct<'a> {
    pub a: i32,
    pub b: &'a str,
}

#[derive(Debug, Encode, Decode, PartialEq, Eq)]
pub struct Person {
    pub id: u32,
    pub age: u8,
}

#[derive(Debug, Encode, Decode, PartialEq, Eq)]
pub struct Company(pub Vec<Person>);

#[derive(Debug, Parser, PartialEq, Eq)]
#[command(name = "utility-demo", about = "Демонстрация разбора аргументов")]
pub struct Args {
    #[arg(short, long)]
    pub string_arg: String,
    #[arg(short, long, default_value_t = true)]
    pub bool_arg: bool,
    #[arg(short, long)]
    pub opt_arg: Option<i32>,
    #[arg(short, long = "list")]
    pub list_arg: Vec<i32>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RequestLine<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub version: &'a str,
    pub status: u16,
}

fn token(input: &str) -> IResult<&str, &str> {
    take_while1(|character: char| !character.is_whitespace())(input)
}

fn status_code(input: &str) -> IResult<&str, u16> {
    map_res(digit1, str::parse).parse(input)
}

pub fn parse_request_line(input: &str) -> IResult<&str, RequestLine<'_>> {
    let (input, (method, _, path, _, http, _, status)) = (
        token,
        space1,
        token,
        space1,
        tag("HTTP/1.1"),
        space1,
        status_code,
    )
        .parse(input)?;
    Ok((
        input,
        RequestLine {
            method,
            path,
            version: http,
            status,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_json_round_trip() {
        let value = MyStruct { a: 42, b: "hello" };
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, r#"{"a":42,"b":"hello"}"#);
        assert_eq!(serde_json::from_str::<MyStruct<'_>>(&json).unwrap(), value);
    }

    #[test]
    fn bincode_round_trip() {
        let company = Company(vec![Person { id: 1, age: 24 }, Person { id: 2, age: 42 }]);
        let config = bincode::config::standard();
        let encoded = bincode::encode_to_vec(&company, config).unwrap();
        let (decoded, read): (Company, usize) =
            bincode::decode_from_slice(&encoded, config).unwrap();
        assert_eq!(decoded, company);
        assert_eq!(read, encoded.len());
    }

    #[test]
    fn nom_parses_request_line() {
        let parsed = parse_request_line("GET /path/get_example HTTP/1.1 200")
            .unwrap()
            .1;
        assert_eq!(
            parsed,
            RequestLine {
                method: "GET",
                path: "/path/get_example",
                version: "HTTP/1.1",
                status: 200,
            }
        );
    }

    #[test]
    fn clap_parses_arguments() {
        let args = Args::try_parse_from([
            "utility-demo",
            "--string-arg",
            "hello",
            "--opt-arg",
            "42",
            "--list",
            "1",
            "--list",
            "2",
        ])
        .unwrap();
        assert_eq!(args.string_arg, "hello");
        assert_eq!(args.opt_arg, Some(42));
        assert_eq!(args.list_arg, [1, 2]);
    }
}
