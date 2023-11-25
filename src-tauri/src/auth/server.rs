use std::io::{BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

static SERVER_ADDRESS: &str = "127.0.0.1:50505";

fn handle_client(stream: TcpStream) -> std::io::Result<(String, String)> {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    let mut buffer = [0; 1024];
    reader.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let params = request.split_whitespace().nth(1).unwrap_or("/");
    let params = &params[1..params.len()];

    let mut code = "";
    let mut state = "";

    for param in params.split(|c| c == '&' || c == '?') {
        if param.is_empty() {
            continue;
        }
        // println!("param: {}", param);
        let parts: Vec<&str> = param.split('=').collect();
        // println!("parts: {:?}", parts);
        if parts.len() == 2 {
            if parts[0] == "code" {
                code = parts[1];
            } else if parts[0] == "state" {
                state = parts[1];
            }
        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid parameter format"));
        }
    }
    let response = if !code.is_empty() {
        format!("HTTP/1.1 200 OK\r\n\r\n{{\"access_token\": \"{}\"}}", code)
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Code is empty"));
    };

    writer.write(response.as_bytes())?;
    writer.flush()?;

    Ok((code.to_string(), state.to_string()))
}

pub fn start_server() -> std::io::Result<(String, String)> {
    // print!("Called Server"); ß
    let listener = TcpListener::bind(SERVER_ADDRESS)?;

    let (tx, rx) = std::sync::mpsc::channel();

    for stream in listener.incoming() {
        let mut stream = stream?;
        let tx = tx.clone();
        thread::spawn(move || -> std::io::Result<()> {
            match handle_client(stream.try_clone()?) {
                Ok((access_token, state)) => {
                    tx.send((access_token, state)).unwrap();
                    Ok(())
                },
                Err(e) => {
                    println!("Error handling client: {}", e);
                    let response = "HTTP/1.1 500 Internal Server Error\r\n\r\n";
                    stream.write(response.as_bytes())?;
                    stream.flush()?;
                    Ok(())
                }
            }
        });
        break; // Break after handling the first connection
    }

    let (access_token, state) = rx.recv().unwrap();

    Ok((access_token, state))
}