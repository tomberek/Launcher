pub mod server;
pub mod xbox;



use crate::auth::xbox::xbox;


pub async fn auth() -> Result<(), Box<dyn std::error::Error>> {
    let (access_token, state) = server::start_server()?;
    println!("Access Token: {}", access_token);
    println!("State: {}", state);

    // Call the xbox function
    println!("Calling xbox function...");
    match xbox(&access_token).await {
        Ok((userhash, xboxtoken)) => {
            println!("Userhash: {}", userhash);
            println!("Xboxtoken: {}", xboxtoken);
            println!("Request was successful");
            Ok(())
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(Box::new(e))
        },




        
    }
    }