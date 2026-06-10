use std::collections::HashMap;

use crate::http::parser::HttpRequest;

pub fn resolve(request: HttpRequest) -> String {
    if request.header.request_line.method == String::from("GET")
        && request.header.request_line.path == String::from("/")
    {
        return String::from("hello server");
    } else if request.header.request_line.method == String::from("GET")
        && request.header.request_line.path == String::from("/health")
    {
        return String::from("web server up");
    } else if request.header.request_line.method == String::from("GET")
        && request.header.request_line.path.contains("/users")
    {
        let mut map = HashMap::new();
        map.insert(1, String::from("Carlos"));
        map.insert(2, String::from("June"));
        map.insert(3, String::from("Antonio"));

        let iter = request.header.request_line.path.split("/users/");
        let last_section = iter.last().unwrap();
        let user_id: u32 = last_section.parse().unwrap();

        if map.contains_key(&user_id) {
            return String::from(map.get(&user_id).unwrap());
        }

        return String::from("No user with such id");
    } else if request.header.request_line.method == String::from("POST")
        && request.header.request_line.path == String::from("/echo")
    {
        return String::from(request.body.content);
    }

    String::new()
}
