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

#[derive(Debug)]
pub struct HttpResponse {
    pub header: ResponseHeader,
    pub body: ResponseBody,
}

#[derive(Debug)]
pub struct ResponseHeader {
    pub start_line: StartLine,
    pub headers: HashMap<String, String>,
}

#[derive(Debug)]
pub struct StartLine {
    pub version: String,
    pub status_code: String,
    pub status_message: String,
}

#[derive(Debug)]
pub struct ResponseBody {
    pub content: String,
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
        if index + 3 < request.len()
            && request[index] == b'\r'
            && request[index + 1] == b'\n'
            && request[index + 2] == b'\r'
            && request[index + 3] == b'\n'
            && request_object.header.headers.contains_key("Content-Length")
        {
            current_request_layer += 1;

            let header = request_object.header.headers.get("Content-Length").unwrap();
            let content_length: usize = header.parse().unwrap();
            let body_start = index + 4;

            let body = String::from_utf8_lossy(&request[body_start..body_start + content_length]);
            request_object.body.content = body.to_string();
        } else if index + 1 < request.len()
            && request[index] == b'\r'
            && request[index + 1] == b'\n'
        {
            let line = String::from_utf8_lossy(&request[line_start..index]);

            if current_request_layer == 0 {
                let mut req_line = line.split_whitespace();
                request_object.header.request_line.method = String::from_iter(req_line.next());
                request_object.header.request_line.path = String::from_iter(req_line.next());
                request_object.header.request_line.version = String::from_iter(req_line.next());

                current_request_layer += 1;
            } else if !line.is_empty() {
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
            .insert(String::from("Content-Length"), content_length.to_string());
        response
            .header
            .headers
            .insert(String::from("Content-Type"), String::from("text/plain"));
    }

    Ok(response)
}

pub fn serialize_response(response: HttpResponse) -> Vec<u8> {
    let mut str = String::new();
    let line_break = String::from("\r\n");
    let header_split = String::from(": ");
    let space = String::from(" ");
    str.push_str(&response.header.start_line.version);
    str.push_str(&space);
    str.push_str(&response.header.start_line.status_code);
    str.push_str(&space);
    str.push_str(&response.header.start_line.status_message);
    str.push_str(&line_break);
    for header in response.header.headers {
        str.push_str(&header.0);
        str.push_str(&header_split);
        str.push_str(&header.1);
        str.push_str(&line_break);
    }
    str.push_str(&line_break);
    str.push_str(&response.body.content);

    Vec::from(str)
}

fn resolve_status_from(status_code: String) -> String {
    if status_code == String::from("200") {
        return String::from("OK");
    }
    if status_code == String::from("404") {
        return String::from("NOT FOUND");
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
