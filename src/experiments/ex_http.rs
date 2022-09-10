extern crate reqwest;
extern crate futures;

use futures::executor::block_on;

/**
 * Testing out different HTTP requests
 */
pub fn experiment_requests() {

    simple_blocking_request();
    complex_blocking_request();

    // Block current thread till an async function completes:
    block_on(make_async_calls());

}

/**
 * A simplified syncronus blocking API request
 */
fn simple_blocking_request()-> String {
    let response_text = reqwest::blocking::get("https://api.coingecko.com/api/v3/ping")
        .expect("Request Failed")
        .text().expect("Request Text Fetch Failed");
    
    println!("Simple blocking request respose: {}", response_text);
    return response_text;
}

/**
 * A more complex syncronus blocking API request
 */
fn complex_blocking_request() {
    match reqwest::blocking::get("https://api.coingecko.com/api/v3/ping") {
        Ok(res) => {
            if res.status() == 200 {
                match res.text() {
                    Ok(c) => println!("Complex blocking request respose: {}", c),
                    Err(_) => println!("Error getting response text.")
                }
            } else {
                println!("Request Failed Status: {}", res.status());
            }
        },
        Err(_) => println!("Could Not Make Request!") 
    }
}

/**
 * A syncronus async API request
 */
async fn simple_async_request() -> String {
    let response_text = reqwest::get("https://api.coingecko.com/api/v3/ping")
        .await.expect("Request Failed")
        .text().await.expect("Request Text Fetch Failed");
    
    println!("Simple async request respose: {}", response_text);
    return response_text;
}

/**
 * A syncronus async API request
 */
async fn complex_async_request() {
    return match reqwest::get("https://api.coingecko.com/api/v3/ping").await {
        Ok(res) => {
            if res.status() == 200 {
                match res.text().await {
                    Ok(c) => println!("Complex async request respose: {}", c),
                    Err(_) => println!("Could Not Make Request!") 
                }
            } else {
                println!("Request Failed Status: {}", res.status());
            }
        },
        Err(_) => println!("Could Not Make Request!") 
    };
    
}

/**
 * A function to call all async methods and await them so they don't block the thread
 */
async fn make_async_calls() {
    // // method 1: implementation would block the second await till the first is done  
    // simple_async_request().await;
    // complex_async_request().await;

    // method 2: perform both awaits without waiting for the first to complete
    let f1 = simple_async_request();
    let f2 = complex_async_request();
    futures::join!(f1, f2);
}