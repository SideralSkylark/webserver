use std::collections::HashMap;

use crate::http::parser::{self, HttpRequest, HttpResponse};

pub fn resolve(request: HttpRequest) -> anyhow::Result<HttpResponse> {
    if request.header.request_line.method == String::from("GET")
        && request.header.request_line.path == String::from("/")
    {
        return parser::create_response(
            String::from("Hello server"),
            String::from("200"),
            String::from("HTTP/1.1"),
        );
    } else if request.header.request_line.method == String::from("GET")
        && request.header.request_line.path == String::from("/health")
    {
        return parser::create_response(
            String::from("Server up"),
            String::from("200"),
            String::from("HTTP/1.1"),
        );
    } else if request.header.request_line.method == String::from("GET")
        && request.header.request_line.path.contains("/users")
    {
        let mut map = HashMap::new();
        map.insert(1, String::from("Carlos"));
        map.insert(2, String::from("June"));
        map.insert(3, String::from("Antonio"));

        let iter = request.header.request_line.path.split("/users/");
        let last_section = iter.last().unwrap();

        match last_section.parse::<u32>() {
            Ok(id) => {
                // look for user
                if map.contains_key(&id) {
                    return parser::create_response(
                        String::from(map.get(&id).unwrap()),
                        String::from("200"),
                        String::from("HTTP/1.1"),
                    );
                }
            }
            Err(_) => {
                return parser::create_response(
                    String::from("Invalid argument"),
                    String::from("400"),
                    String::from("HTTP/1.1"),
                );
            }
        }

        return parser::create_response(
            String::from("No user with such id"),
            String::from("404"),
            String::from("HTTP/1.1"),
        );
    } else if request.header.request_line.method == String::from("POST")
        && request.header.request_line.path == String::from("/echo")
    {
        return parser::create_response(
            String::from(request.body.content),
            String::from("200"),
            String::from("HTTP/1.1"),
        );
    }

    parser::create_response(
        String::from("Resource not found"),
        String::from("404"),
        String::from("HTTP/1.1"),
    )
}
