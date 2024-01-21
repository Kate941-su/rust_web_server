use std::{
    // To handle an os file system.
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use webserver::ThreadPool;

fn main() {
    let binding_port = "7878";
    // let listener = TcpListener::bind("127.0.0.1:".to_owned() + binding_port).unwrap();
    let listener = match TcpListener::bind("127.0.0.1:".to_owned() + binding_port) {
        Ok(listener) => {
            println!("You could bind the port: {}", binding_port);
            listener
        }
        Err(err) => {
            panic!(
                "You could not bind the port: {} because of {}",
                binding_port, err
            );
        }
    };
    let pool = ThreadPool::new(4);
    // The `take` method is defined int the `Iterator` trait and limits
    // the iteration to the first two items at most.
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        // If you write the following spawn, you may create new threads without any limit !
        // thread::spawn(||{
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // You can define variables by using tuple.
    let (status_line, file_name) = match &request_line[..] {
        // The following request would match 'http://localhost/'
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "html/hello.html"),
        // The following request would match 'http://localhost/sleep'
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "html/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "html/404.html"),
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
