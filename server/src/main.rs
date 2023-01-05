fn main() {
    let server = Server::new("127.0.0.1:3000");
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
