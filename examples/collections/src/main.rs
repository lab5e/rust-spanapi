use std::env;
use futures::executor::block_on;
use spanapi::apis::configuration;
use spanapi::apis::configuration::ApiKey;
use spanapi::apis::collections_api::{list_collections};

#[tokio::main]
async fn main() {
    // Get the token from the command line.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Needs a token")
    }

    // Create the api key struct for the token
    let api_key = ApiKey{
        prefix: None,
        key: String::from(&args[1]),
    };

    // Create configuration and add the API key struct to it.
    let mut config = configuration::Configuration::new();
    config.api_key = Some(api_key);

    // Finally -- get the profile
    let collection_result = list_collections(&config);

    // ..and print the result
    match block_on(collection_result) {
        Ok(resp) => {
            println!("Collections");
            println!("=======");
            match resp.collections {
                Some(colls) => {
                    for n in 0..colls.len() {
                        match &colls[n].collection_id {
                            Some(id) => println!("Collection ID: {}",id),
                            None=> println!("Collection ID is empty!"),
                        }
                    }
                },
                None => println!("No collection list returned!"),
            }
        },
        Err(error) => {
            println!("error: {:?}", error);
            panic!("Query failed!")
        },
    }
}
