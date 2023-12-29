use server::Server;
use http::request::Request;

mod server {
    pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Self { addr }
        }

        pub fn run(self) {
            println!("Listening on {}", self.addr)
        }
    }
}

mod http {
    pub mod request {
        use super::request_method::RequestMethod;
        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: RequestMethod,
        }
    }

    mod request_method {
        pub enum RequestMethod {
            GET,
            POST,
            PUT,
            HEAD,
            DELETE,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH,
        }
    }
}

fn main() {
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();
}
