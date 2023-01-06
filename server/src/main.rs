use std::string;

fn main() {
    let string = String::from("127.0.0.1:3000");

    let sliced_string = &string[10..];

    let server = Server::new(string);

    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {}
}
