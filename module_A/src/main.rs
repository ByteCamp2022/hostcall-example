wit_bindgen_rust::import!("../imports.wit");
wit_bindgen_rust::export!("../exports.wit");

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

}

fn main() {
    println!("from module main");
}
