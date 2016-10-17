extern crate slack_api;
extern crate hyper;

use std::io::prelude::*;
use std::io::Result;
use std::fs::File;

fn main() {
    let client = hyper::client::Client::new();
    let client_id = auth_file_contents("CLIENT_ID").unwrap();
    let client_secret = auth_file_contents("CLIENT_SECRET").unwrap();
    let access_response = slack_api::oauth::access(&client,
                                                   &client_id,
                                                   &client_secret,
                                                   "CODE",
                                                   None)
                              .expect("OAuth access request failed");
    match slack_api::channels::list(&client, &access_response.access_token, None) {
        Ok(list_response) => {
            for channel in list_response.channels {
                println!("channel: {}", channel.name);
            }
        }
        Err(err) => println!("{}", err),
    }
}

fn auth_file_contents(filename: &str) -> Result<String> {
    let mut file = File::open(&format!("../auth/{}", filename))
                       .expect(&format!("{} file not found", filename));
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return Ok(contents);
}
