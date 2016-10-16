extern crate slack_api;
extern crate hyper;

fn main() {
    let client = hyper::client::Client::new();
    match slack_api::channels::list(&client, "TEST_TOKEN", None) {
        Ok(list_response) => {
            for channel in list_response.channels {
                println!("channel: {}", channel.name);
            }
        }
        Err(err) => println!("{}", err)
    }
}
