use reqwest::Error;
mod helpers;
use helpers::{read_file_lines_to_vec};

async fn get_request() -> Result<(), Error> {

    let file_path = "./urls.txt";
    let url_vector = read_file_lines_to_vec(&file_path.to_string());
    match &url_vector {
        // If the operation was successful, make requests to urls in the file.
        Ok(file_contents) => {
            for url in file_contents {
                let response = reqwest::get(url).await?;
                println!("Status: {}", response.status());

                let body = response.text().await?;
                println!("Body:\n{}", body);
            }
        }

        // If the operation failed, print the error message to the console.
        Err(error) => {
            println!("Error reading file: {}", error);
        }
    }
    Ok(())
    
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let file_path = "./urls.txt";
    let url_vector = read_file_lines_to_vec(&file_path.to_string());
    
    println!("{:?}", url_vector);
    get_request().await?;
    Ok(())
}