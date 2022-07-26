use crate::http::{ParseError, Request, Response, StatusCode, Method};
use super::server::Handler;

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        // Response::new(StatusCode::Ok, Some("<h1>Good request</h1>".to_string()))
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h2>Welcome</h2>".to_string())),
                "/hello" => Response::new(StatusCode::Ok, Some("<h2>hello</h2>".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            }
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}