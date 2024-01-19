use std::{
    // To handle an os file system.
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}};

fn main() {
    let binding_port = "7878";
    // let listener = TcpListener::bind("127.0.0.1:".to_owned() + binding_port).unwrap();
    let listener = match TcpListener::bind("127.0.0.1:".to_owned() + binding_port) {
        Ok(listener) => {
            println!("You could bind the port: {}", binding_port);
            listener
        }
        Err(err) => {
            panic!("You could not bind the port: {} because of {}", binding_port, err);
        }
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // You can define variables by using tuple.
    let (status_line, file_name) = if request_line == "GET / HTTP/1.1" {
         ("HTTP/1.1 200 OK", "html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "html/404.html")
    };
    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_connection_old(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    // println!("Request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    // fs reads from root directory.
    // If you put it `hello.html` in the src directory,
    // you should write "src/hello.html" as a path.
    let contents = fs::read_to_string("html/hello.html").unwrap();
    let length = contents.len();
    // we use format! to add the file’s contents as the body of the success response.
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
