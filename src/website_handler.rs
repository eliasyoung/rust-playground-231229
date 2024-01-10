use super::http::{Request, RequestMethod, Response, StatusCode};
use super::server::Handler;
pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> WebsiteHandler {
        WebsiteHandler { public_path }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            RequestMethod::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>Hello World</h1>".to_string())),
                "/hello" => Response::new(StatusCode::Ok, Some("<h1>Hello Page</h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
