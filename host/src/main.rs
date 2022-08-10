// use anyhow::Context;
use anyhow::Result;
use wasmtime::*;

use std::net::TcpListener;
use std::io::{prelude::*, self, BufReader, Write};
use std::net::TcpStream;

wit_bindgen_wasmtime::export!("../imports.wit");
wit_bindgen_wasmtime::import!("../exports.wit");

use imports::*;
use exports::*;


#[derive(Default)]
pub struct MyImports;


impl Imports for MyImports {
    fn hostf1(&mut self, s: &str) {
        println!("{}", s);
    }

    fn hostnet(&mut self) -> String {
        // let lis = TcpListener::bind("127.0.0.1:7878").unwrap();

        // for stream in lis.incoming() {
        //     let mut stream = stream.unwrap();

        //     let mut buffer = [0; 1024];

        //     stream.read(&mut buffer).unwrap();

        //     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        // }

        let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
        
        for _ in 0..10 {
            let mut req = String::new();
            io::stdin().read_line(&mut req).expect("read fail");
            stream.write(req.as_bytes()).expect("fail");

            let mut reader = BufReader::new(&stream);
            let mut buffer: Vec<u8> = Vec::new();
            reader.read_until(b'\n', &mut buffer).expect("fail");

            println!("read from server: {}", std::str::from_utf8(&buffer).unwrap());
            println!("");
        }

        return "nice".into();
    }

}

struct Context<I, E> {
    wasi: wasmtime_wasi::WasiCtx,
    imports: I,
    exports: E,
}


fn default_config() -> Result<Config> {
    // Create an engine with caching enabled to assist with iteration in this
    // project.
    let mut config = Config::new();
    config.cache_config_load_default()?;
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    Ok(config)
}

fn default_wasi() -> wasmtime_wasi::WasiCtx {
    wasmtime_wasi::sync::WasiCtxBuilder::new()
        .inherit_stdio()
        .build()
}


fn instantiate<I: Default, E: Default, T>(
    wasm: &str,
    add_imports: impl FnOnce(&mut Linker<Context<I, E>>) -> Result<()>,
    mk_exports: impl FnOnce(
        &mut Store<Context<I, E>>,
        &Module,
        &mut Linker<Context<I, E>>,
    ) -> Result<(T, Instance)>,
) -> Result<(T, Store<Context<I, E>>)> {
    let engine = Engine::new(&default_config()?)?;
    let module = Module::from_file(&engine, wasm)?;

    let mut linker = Linker::new(&engine);
    add_imports(&mut linker)?;
    wasmtime_wasi::add_to_linker(&mut linker, |cx| &mut cx.wasi)?;

    let mut store = Store::new(
        &engine,
        Context {
            wasi: default_wasi(),
            imports: I::default(),
            exports: E::default(),
        },
    );
    let (exports, _instance) = mk_exports(&mut store, &module, &mut linker)?;
    Ok((exports, store))
}






fn main() -> Result<()> {

    let (exports, mut store) = crate::instantiate(
        "module_net.wasm",
        |linker| imports::add_to_linker(linker, |cx| -> &mut MyImports { &mut cx.imports }),
        |store, module, linker| Exports::instantiate(store, module, linker, |cx| &mut cx.exports),
    )?;
    exports.modulef1(&mut store, "sdf")?;
    let s2 = exports.modulef2(&mut store)?;
    println!("{}", s2);
    exports.modulef3(&mut store)?;
    exports.modulef4(&mut store, &[1, 2, 3, 4])?;
    exports.modulenet(&mut store)?;
    
    Ok(())
}
