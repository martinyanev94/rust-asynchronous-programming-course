fn get_req(path: &str) -> Vec<u8> {
    format!(
        "GET {path} HTTP/1.1\r\n\
         Host: localhost\r\n\
         Connection: close\r\n\
         \r\n"
    ).into_bytes()
}
