use reqwest::Error;

pub async fn get_request(url: String) -> Result<(), Error> {
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

pub async fn post_request(url: String, json_data: String) -> Result<(), Error> {
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

pub async fn put_request(url: String, json_data: String) -> Result<(), Error> {
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

pub async fn delete_request(url: String) -> Result<(), Error> {
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