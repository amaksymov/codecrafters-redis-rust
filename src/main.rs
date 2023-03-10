// Uncomment this block to pass the first stage
use std::io::{Write, BufReader, BufRead, Read};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // let buf_reader = BufReader::new(&mut stream);
    // let request: Vec<_> = buf_reader.lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // println!("{:?}", request);

    let response = "+PONG\r\n";
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 { 
                    // connection was closed
                    break;
                }
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379")?;

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_client(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
