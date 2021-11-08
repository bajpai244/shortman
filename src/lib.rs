use url::{ParseError, Url};
mod shortner;
use shortner::Shortener;
use std::env;

pub fn get_id(url: String) -> String {
    let connection_url = env::var("CONNECTION_URL").expect("environment variable is not present");
    let mut sh = Shortener::new(connection_url);
    sh.next_id(url).to_string()
}

pub fn get_val(key: String) -> String {
    let connection_url = env::var("CONNECTION_URL").expect("environment variable is not present");
    let mut sh = Shortener::new(connection_url);
    sh.get_value(key)
}

pub fn is_url_val(url: &str) -> Result<(), String> {
    let parse_result = Url::parse(url);

    let error_val = (
        String::from("the given url is not valid! please provide a valid url!"),
        String::from("Something went wrong make sure that you are providing a valid url! If the error persists feel free to reach out to the author!"),
    );

    match parse_result {
        Ok(_) => {}
        Err(ParseError::InvalidIpv4Address) => {
            return Err(error_val.0);
        }
        _ => {
            return Err(error_val.1);
        }
    }

    Ok(())
}
