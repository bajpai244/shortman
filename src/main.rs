#![feature(proc_macro_hygiene, decl_macro)]

use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::response::{content, status, Redirect};
use rocket::Outcome;
use rocket::*;
use rocket_contrib::json::Json;
use serde::Deserialize;
use serde::Serialize;

use rckt::{get_id, get_val, is_url_val};

#[get("/route_404")]
fn four_o_four() -> Status {
    Status::NotFound
}

#[get("/<id>")]
fn index(id: String) -> Redirect {
    let url = get_val(id);

    println!("val is {}", url);

    if url != "NULL" {
        return Redirect::to(url);
    } else {
        return Redirect::to(uri!(four_o_four));
    }
}

#[derive(Serialize, Responder)]
#[response(status = 400, content_type = "json")]
struct BadRequest {
    error: String,
}

#[derive(Serialize, Responder)]
#[response(status = 201, content_type = "json")]
struct Success {
    url: String,
}

#[derive(Serialize, Responder)]
enum Resp {
    BadRequest(BadRequest),
    Success(Success),
}

#[derive(Debug)]
pub struct HostHeader<'a>(pub &'a str);

impl<'a, 'r> FromRequest<'a, 'r> for HostHeader<'a> {
    type Error = ();

    fn from_request(request: &'a Request) -> rocket::request::Outcome<Self, Self::Error> {
        match request.headers().get_one("Host") {
            Some(h) => Outcome::Success(HostHeader(h)),
            None => Outcome::Forward(()),
        }
    }
}

#[post("/", format = "json", data = "<body>")]
fn url_route(body: Json<String>, header: HostHeader) -> Resp {
    let url: String;
    let short_url: String;

    if let Json(val) = body {
        url = val;
        let result = is_url_val(&url);

        match result {
            Ok(val) => {
                let url_id = get_id(url.clone());
                return Resp::Success(Success { url: url_id });
            }
            Err(err) => {
                return Resp::BadRequest(BadRequest { error: err });
            }
        }
    } else {
        return Resp::BadRequest(BadRequest {
            error: "The body of the request is not properly formatted!".to_string(),
        });
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, url_route, four_o_four])
        .launch();
}
