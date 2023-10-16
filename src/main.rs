use reqwest::Error;
mod helpers;
//use helpers::{read_file_lines_to_vec};

use serde::Deserialize;
use std::fs;
use serde_json::Value;

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

    
    //let config = parse_toml_config()?;
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


    if data.config.method == "DELETE" {
        delete_request(data.config.url).await;
    } else if data.config.method == "POST" {
        post_request(data.config.url, body.to_string()).await;

    } else if data.config.method == "PUT" {
       put_request(data.config.url, body.to_string()).await;
    } else {
        
        get_request(data.config.url).await;
    }

    

    
    Ok(())

    
}


async fn get_request(url: String) -> Result<(), Error> {
    /*
    let file_path = "./urls.txt";
    let url_vector = read_file_lines_to_vec(&file_path.to_string());
    match &url_vector {
        // If the operation was successful, make requests to urls in the file.
        Ok(file_contents) => {
            for url in file_contents {
                let response = reqwest::get(url).await?;
                println!("Status code: {}", response.status());

                let body = response.text().await?;
                println!("Response body:\n{}", body);
            }
        }

        // If the operation failed, print the error message to the console.
        Err(error) => {
            println!("Error reading file: {}", error);
        }
    }
    */

    let response = reqwest::get(url).await?;
    println!("Status code: {}", response.status());

    let body = response.text().await?;
    println!("Response body:\n{}", body);
    Ok(())
    
}

async fn post_request(url: String, json_data: String) -> Result<(), Error> {
    //let url = "http://localhost:4000/tasks";
    //let json_data = r#"{"title":"Problems during installation","status":"todo","priority":"medium","label":"bug"}"#;

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(json_data.to_owned())
        .send()
        .await?;
    
    println!("Status code: {}", response.status());

    let response_body = response.text().await?;

    println!("Response body: \n{}", response_body);

    Ok(())

}

async fn put_request(url: String, json_data: String) -> Result<(), Error> {
    //let url = "http://localhost:4000/tasks/7";
    //let json_data = r#"{"title":"Problems during installation","status":"todo","priority":"low","label":"bug"}"#;

    let client = reqwest::Client::new();

    let response = client
        .put(url)
        .header("Content-Type", "application/json")
        .body(json_data.to_owned())
        .send()
        .await?;
    
    println!("Status code: {}", response.status());

    let response_body = response.text().await?;

    println!("Response body: \n{}", response_body);

    Ok(())
}

async fn delete_request(url: String) -> Result<(), Error> {
    //let url = "http://localhost:4000/tasks/5";

    let client = reqwest::Client::new();

    let response = client
        .delete(url)
        .send()
        .await?;
    
    println!("Status code: {}", response.status());

    let response_body = response.text().await?;

    println!("Response body: \n{}", response_body);

    Ok(())
}