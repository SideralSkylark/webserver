use crate::http::parser::HttpRequest;

pub fn resolve(request: HttpRequest) -> String {
    if request.header.request_line.method == String::from("GET")
        && request.header.request_line.path == String::from("/")
    {
        return String::from("hello server");
    }

    String::new()
}
