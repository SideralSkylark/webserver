use std::collections::HashMap;

use anyhow::Ok;

#[derive(Debug)]
pub struct HttpRequest {
    pub header: RequestHeader,
    pub body: RequestBody,
}

#[derive(Debug)]
pub struct RequestHeader {
    pub request_line: RequestLine,
    pub headers: HashMap<String, String>,
}

#[derive(Debug)]
pub struct RequestLine {
    pub method: String,
    pub path: String,
    pub version: String,
}

#[derive(Debug)]
pub struct RequestBody {
    pub content: String,
}

pub struct HttpResponse {
    header: ResponseHeader,
    body: ResponseBody,
}

pub struct ResponseHeader {
    start_line: StartLine,
    headers: HashMap<String, String>,
}

pub struct StartLine {
    version: String,
    status_code: String,
    status_message: String,
}

pub struct ResponseBody {
    content: String,
}

// for requests there is the request line + header (can be n headers) + body (separated from the
// other two by an empty line b"\r\n\r\n")
pub fn parse_request(request: &mut [u8]) -> anyhow::Result<HttpRequest> {
    let mut index = 0;
    let mut line_start = 0;
    let mut current_request_layer = 0;
    let mut request_object: HttpRequest = HttpRequest {
        header: RequestHeader {
            request_line: RequestLine {
                method: String::new(),
                path: String::new(),
                version: String::new(),
            },
            headers: HashMap::new(),
        },
        body: RequestBody {
            content: String::new(),
        },
    };

    while index < request.len() {
        if index + 1 < request.len() && request[index] == b'\r' && request[index + 1] == b'\n' {
            let line = String::from_utf8_lossy(&request[line_start..index]);

            if current_request_layer == 0 {
                let mut req_line = line.split_whitespace();
                request_object.header.request_line.method = String::from_iter(req_line.next());
                request_object.header.request_line.path = String::from_iter(req_line.next());
                request_object.header.request_line.version = String::from_iter(req_line.next());

                current_request_layer += 1;
            } else if line.is_empty() {
                current_request_layer += 1;
                let body = String::from_utf8_lossy(&request[index + 2..]);
                request_object.body.content = body.to_string();
            } else {
                let mut header_line = line.split(": ");
                request_object.header.headers.insert(
                    String::from_iter(header_line.next()),
                    String::from_iter(header_line.next()),
                );
            }
            index += 2;
            line_start = index;
            continue;
        }

        index += 1;
    }

    if is_malformed(&request_object) {
        println!("invalid request format");
    }

    Ok(request_object)
}

// for now just check request line, later check headers and given content related headers verify if
// the body matches
fn is_malformed(request: &HttpRequest) -> bool {
    if request.header.request_line.method.is_empty()
        || request.header.request_line.path.is_empty()
        || request.header.request_line.version.is_empty()
    {
        return true;
    }

    false
}

pub fn create_response(
    message: String,
    status_code: String,
    version: String,
) -> anyhow::Result<HttpResponse> {
    let mut content_length = 0;
    if !message.is_empty() {
        content_length = message.len()
    }
    let mut response = HttpResponse {
        header: ResponseHeader {
            start_line: StartLine {
                version: version,
                status_code: status_code.clone(),
                status_message: resolve_status_from(status_code),
            },
            headers: HashMap::new(),
        },
        body: ResponseBody { content: message },
    };

    if content_length > 0 {
        response
            .header
            .headers
            .insert(String::from("Content-Lenght"), content_length.to_string());
        response
            .header
            .headers
            .insert(String::from("Content-Type"), String::from("text/plain"));
    }

    Ok(response)
}

fn resolve_status_from(status_code: String) -> String {
    if status_code == String::from("200") {
        return String::from("ok");
    }
    if status_code == String::from("404") {
        return String::from("not found");
    }
    if status_code == String::from("500") {
        return String::from("internal server error");
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_malformed_no_method() {
        let request: HttpRequest = HttpRequest {
            header: RequestHeader {
                request_line: RequestLine {
                    method: String::new(),
                    path: String::from("/"),
                    version: String::from("HTTP/1.1"),
                },
                headers: HashMap::new(),
            },
            body: RequestBody {
                content: String::new(),
            },
        };

        assert_eq!(is_malformed(&request), true)
    }

    #[test]
    fn is_malformed_no_path() {
        let request: HttpRequest = HttpRequest {
            header: RequestHeader {
                request_line: RequestLine {
                    method: String::from("GET"),
                    path: String::new(),
                    version: String::from("HTTP/1.1"),
                },
                headers: HashMap::new(),
            },
            body: RequestBody {
                content: String::new(),
            },
        };

        assert_eq!(is_malformed(&request), true)
    }

    #[test]
    fn is_malformed_no_version() {
        let request: HttpRequest = HttpRequest {
            header: RequestHeader {
                request_line: RequestLine {
                    method: String::from("GET"),
                    path: String::from("/"),
                    version: String::new(),
                },
                headers: HashMap::new(),
            },
            body: RequestBody {
                content: String::new(),
            },
        };

        assert_eq!(is_malformed(&request), true)
    }
}
