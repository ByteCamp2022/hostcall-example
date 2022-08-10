### 测试网络相关

exports.wit添加

```
modulenet: func()
```

module_net/src/main.rs中添加

```rust
fn modulenet() {
        println!("TcpListener Test");
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        println!("It works.");
    }
```

host/src/main.rs中添加

```rust
 exports.modulenet(&mut store)?;
```

运行host

报错

```shell
message: sdf
implemented in host
sdf
implemeted in module
first 1
TcpListener Test
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: Error { kind: Unsupported, message: "operation not supported on this platform" }', src\main.rs:28:60
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Error: wasm trap: wasm `unreachable` instruction executed
wasm backtrace:
    0: 0x8534 - panic_abort::__rust_start_panic::abort::h942bc19248e02a5a
                    at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/panic_abort/src/lib.rs:85:17
              - __rust_start_panic
                    at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/panic_abort/src/lib.rs:38:5
    1: 0x82e4 - rust_panic
                    at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/std/src/panicking.rs:746:9
    2: 0x8261 - std::panicking::rust_panic_with_hook::h0934220b8bcb5bb0
                    at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/std/src/panicking.rs:716:5
    3: 0x74e4 - std::panicking::begin_panic_handler::{{closure}}::hd757106e895f8529
                    at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/std/src/panicking.rs:588:13
    4: 0x7423 - std::sys_common::backtrace::__rust_end_short_backtrace::hda2217248ddf29b4
                    at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/std/src/sys_common/backtrace.rs:138:18
    5: 0x7b8c - rust_begin_unwind
                    at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/std/src/panicking.rs:584:5
    6: 0xd4b7 - core::panicking::panic_fmt::h99615bf411b9c1d7
                    at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:142:14
    7: 0xf0c5 - core::result::unwrap_failed::h42eb2bde7057019e
                    at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/result.rs:1785:5
    8:  0x511 - <unknown>!<module_net::Exports as module_net::exports::Exports>::modulenet::h6545226d82147e9d
    9:  0x869 - <unknown>!modulenet
   10: 0x1184b - <unknown>!modulenet.command_export

error: process didn't exit successfully: `target\debug\host.exe` (exit code: 1)
```

