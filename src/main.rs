extern crate iron;

#[macro_use]
extern crate serde_json;
extern crate serde;

use iron::prelude::*;
use iron::headers::ContentType;
use iron::status;
use std::io::Read;
use serde_json::{Value};

fn webhook(req: &mut Request) -> IronResult<Response> {
    let mut body = String::new();

    req.body.read_to_string(&mut body).unwrap();

    let query: Value = serde_json::from_str(&body).unwrap();
    let result = &query["result"];
    let action = &result["action"];
    let parameters = &result["parameters"];
    let color = parameters["color"].as_str().unwrap();
    let number: String = if parameters["number"].is_string() { 
        parameters["number"].as_str().unwrap().to_string()
    } else { 
        parameters["number"].to_string() 
    };

    println!("Received query with params {} and {}", color, number);

    if action == "make.name" {
        let text = format!("Alright, your silly name is {} {}! I hope you like it. See you next time.", color, number);

        let response_body = json!({
            "speech": text,
            "displayText": text
        });

        Ok(Response::with((ContentType::json().0, status::Ok, response_body.to_string())))
    } else {
        Ok(Response::with(status::ImATeapot))
    }
}

fn main() {
    println!("starting up on port 8080");
    Iron::new(webhook).http("0.0.0.0:8080").unwrap();
}
