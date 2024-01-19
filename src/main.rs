use std::net::TcpListener;

fn main() {
    let binding_port = "80";
    // let listener = TcpListener::bind("127.0.0.1:".to_owned() + binding_port).unwrap();
    let listener = match TcpListener::bind("127.0.0.1:".to_owned() + binding_port) {
        Ok(listener) => {
            println!("You could bind the port: {}", binding_port);
            listener
        },
        Err(err) => {
            panic!("You could not bind the port: {} because of {}", binding_port, err);
        },
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
    }
}
