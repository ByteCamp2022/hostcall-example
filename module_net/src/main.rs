wit_bindgen_rust::import!("../imports.wit");
wit_bindgen_rust::export!("../exports.wit");

use std::net::TcpListener;

struct Exports;

impl exports::Exports for Exports {
    fn modulef1(s: String) {
        println!("message: {}", s);
        imports::hostf1("implemented in host");
    }

    fn modulef2() -> String {
        "sdf".into()
    }

    fn modulef3() {
        println!("implemeted in module");
    }

    fn modulef4(slice: Vec<u8>) {
        println!("first {}", slice[0]);
    }

    fn modulenet() {
        println!("TcpListener Test");
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        println!("It works.");
    }
}

fn main() {
    println!("from module main");
}
