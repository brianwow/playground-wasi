// Copied example code from wasmedge_http_req
use wasmedge_http_req::request;

fn main() {
    let mut writer = Vec::new();
    let res = request::get("http://doc.rust-lang.org/", &mut writer).unwrap();

    println!("Status: {} {}", res.status_code(), res.reason());
}
