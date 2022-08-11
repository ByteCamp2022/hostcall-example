# hostcall-example
with wit-bindgen

# 热加载, 热更新
module_B 覆盖 module_A

1.进入module_A目录
```
make build
cp target/wasm32-wasi/release/module_A.wasm ../host/
```
2.进入module_B目录
```
make build
cp target/wasm32-wasi/release/module_B.wasm ../host/
```


2.进入host目录
```
make build
make run
```

output
```
enter module a, message: "call after first load"
enter host f1, message: "implemented in host"

enter module b, message: "call after second load"
enter host f1, message: "implemented in host"

enter module b
Hi, "John Doe"
Now, you are 43 old
You must be 48 old in 2027
```

# 函数模板
## host side
```rust
fn f1(v: &serde_json::Value) -> serde_json::Value {
    println!("enter host f1, message: {}", v["message"]);
    println!("");
    let rs = json!({
        "message": "ok",
      });
    rs
}
```

## module side
```rust
fn modulef1(s: &serde_json::Value) -> serde_json::Value{
    println!("enter module b, message: {}", s["message"]);
    // call func from host side
    imports::proxy("f1", &json!({"message": "implemented in host",}).to_string());
    json!({"message": "modulef1",})
}
```
