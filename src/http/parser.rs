// pub struct http_request {
//     header: ,
//     body
// }

pub fn parse_request(request: &mut [u8]) {
    for byte in request {
        println!("{:?}", byte);
    }
}
