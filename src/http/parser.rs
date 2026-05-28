use std::collections::HashMap;

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

// for requests there is the request line + header (can be n headers) + body (separated from the
// other two by an empty line b"\r\n\r\n")
pub fn parse_request(request: &mut [u8]) {
    let mut index = 0;
    let mut break_counter = 0;
    let layers = 3;
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
            break_counter += 1;

            if break_counter == 1 {
                // request_object.header.method =
                //     String::from_utf8_lossy(&request[..index]).to_string();
                // println!("{:?}", request_object);
            };

            index += 2;

            continue;
        }

        index += 1;
    }
}
