# hostcall-example
with wit-bindgen

1.进入module_A目录
```
make build
cp target/wasm32-wasi/release/module_A.wasm ../host/
```
2.进入host目录
```
make build
make run
```

output
```
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/host`
message: sdf
implemented in host
```
