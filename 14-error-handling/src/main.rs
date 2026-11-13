use std::error::Error;

use crate::error::{DownloadError, GetDataError, LoginError};

mod error;

fn main() {
    println!("Correct case:");
    let result = get_data("a@b.com", "password", "key");
    if let Err(e) = result {
        process_error(&e);
    }

    println!();
    println!("Bas email case:");
    let result = get_data("aaaaa@b.com", "password", "key");
    if let Err(e) = result {
        process_error(&e);
    }

    println!();
    println!("Bad pass case:");
    let result = get_data("a@b.com", "password123123", "key");
    if let Err(e) = result {
        process_error(&e);
    }

    println!();
    println!("Bad key case:");
    let result = get_data("a@b.com", "password", "key123123");
    if let Err(e) = result {
        process_error(&e);
    }
}

fn process_error(e: &impl Error) {
    println!("error: {}", e);
    eprintln!("error details: {:?}", e);

    if let Some(source) = e.source() {
        eprintln!("error source: {}", source);
    }
}

pub fn get_data(email: &str, pass: &str, key: &str) -> Result<String, GetDataError> {
    // let user_id = match login(email, pass) {
    //     Ok(user_id) => user_id,
    //     Err(e) => return Err(GetDataError::Login(e)),
    // };

    // let data = match download(&user_id, key) {
    //     Ok(data) => data,
    //     Err(e) => return Err(GetDataError::Download(e)),
    // };

    let user_id = login(email, pass)?;
    let data = download(&user_id, key)?;

    Ok(data)
}

pub fn login(email: &str, pass: &str) -> Result<String, LoginError> {
    if email != "a@b.com" {
        return Err(LoginError::IncorrectEmail);
    }

    if pass != "password" {
        return Err(LoginError::IncorrectPassword);
    }

    Ok("user_id".to_string())
}

pub fn download(user_id: &str, key: &str) -> Result<String, DownloadError> {
    if key != "key" {
        return Err(DownloadError::IncorrectKey);
    }

    if user_id != "user_id" {
        return Err(DownloadError::IncorrectUserId);
    }

    Ok("data".to_string())
}
