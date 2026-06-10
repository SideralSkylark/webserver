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
    } else if request.header.request_line.method == String::from("POST")
        && request.header.request_line.path == String::from("/echo")
    {
        return String::from(request.body.content);
    }

    String::new()
}
