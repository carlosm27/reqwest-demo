use reqwest::Error;
mod helpers;
//use helpers::{read_file_lines_to_vec};

use serde::Deserialize;
use std::fs;
use serde_json::Value;

mod http_methods;
use http_methods::{get_request, post_request, put_request, delete_request};


#[derive(Deserialize)]
struct Data {
   config: Config,
}
#[derive(Deserialize)]
struct Config {
    url: String,
    method: String,
}




#[tokio::main]
async fn main()-> Result<(), Error> {
    //let file_path = "./urls.txt";
   // let url_vector = read_file_lines_to_vec(&file_path.to_string());
    
   // println!("{:?}", url_vector);

    
   
    let filename = "config.toml";
    
    let contents = match fs::read_to_string(filename) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(error) => {
            // Write `msg` to `stderr`.
            (&error).to_string()
            // Exit the program with exit code `1`.
        
        }
    };

    let data: Data = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(error) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data because `{}`", error);
            // Exit the program with exit code `1`.
            std::process::exit(1);
        }
    };

    let body = {
        let file_content = fs::read_to_string("./body.json").expect("Error reading file");
        serde_json::from_str::<Value>(&file_content).expect("Error serializing to JSON")
    };

    /*
    if data.config.method == "DELETE" {
        delete_request(data.config.url).await;
    } else if data.config.method == "POST" {
        post_request(data.config.url, body.to_string()).await;

    } else if data.config.method == "PUT" {
       put_request(data.config.url, body.to_string()).await;
    } else {
        
        get_request(data.config.url).await;
    }
    */

 
    let result = method_control(&data.config.method, data.config.url, body.to_string()).await;

    match result {
        Ok(contents) => contents,
        Err(e) => println!("Error during the request: {}", e),
    }
    Ok(())

    
}

async fn method_control(http_method: &str, url: String, body: String) -> Result<(), reqwest::Error> {

    match http_method {
        "POST" => post_request(url, body).await,
        "PUT" => put_request(url, body).await,
        "DELETE" => delete_request(url).await,
        _ => get_request(url).await,

    }
}

