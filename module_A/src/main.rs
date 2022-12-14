use serde::{Deserialize, Serialize};
use serde_json::json;

wit_bindgen_rust::import!("../imports.wit");
wit_bindgen_rust::export!("../exports.wit");

struct Exports;

impl exports::Exports for Exports {

    fn proxy(name: String, param: String) -> String{
        let v:serde_json::Value = serde_json::from_str(&param.as_str()).unwrap();
        match name.as_str() {
            "modulef1" => {
                return modulef1(&v).to_string();
            }
            "modulef2" => {
                return modulef2(&v).to_string();
            }
            "modulef3" => {
                return modulef3(&v).to_string();
            }
            _ => return json!({"message": "None",}).to_string(),
    }
    }

}

fn modulef1(s: &serde_json::Value) -> serde_json::Value{
    println!("enter module a, message: {}", s["message"]);
    imports::proxy("f1", &json!({"message": "implemented in host",}).to_string());
    json!({"message": "modulef1",})
}

fn modulef2(v: &serde_json::Value) -> serde_json::Value{
    println!("Hi, {}", v["name"]);
    json!({"message": "modulef2",})

}

fn modulef3(s: &serde_json::Value) -> serde_json::Value{
    println!("implemeted in module");
    json!({"message": "modulef3",})

}

fn main() {
    println!("from module main");
}
