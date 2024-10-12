use serde_json::{json,Value};

use http::{Request, Response};


fn regauth(username: String,password: String, email: String,server_address: String) {
    /* allows the TUI to access the registry*/
    let credential: Value = json!({
        "username":username,
        "password":password,
        "email": email,
        "server_address": server_address
    });
    let credentials = credential.as_str().unwrap();

    let accessreg = Request::builder()
                                .method("POST")
                                .uri("https://docker.com/v1.47/auth")
                                .body(credentials); 
    
    match accessreg {
        Ok(_) => println!("Identity token ({:#?}) was recieved. ",accessreg.unwrap()),
        Err(_) => println!("Accessing identity token failed. Error: {:#?}",accessreg.unwrap_err())
    };
    
}