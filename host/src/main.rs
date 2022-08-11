use std::{collections::HashMap, hash::Hash};
use lazy_static::*;
use serde_json::json;
use std::sync::Mutex;
// use anyhow::Context;
use anyhow::Result;
use wasmtime::{*};

use serde::{Deserialize, Serialize};
// use serde_json::Result;

wit_bindgen_wasmtime::export!("../imports.wit");
wit_bindgen_wasmtime::import!("../exports.wit");


use imports::*;
use exports::*;


// ----------------- 实际实现 --------------
fn f1(v: &serde_json::Value) -> serde_json::Value {
    println!("enter host f1, message: {}", v["message"]);
    println!("");
    let rs = json!({
        "message": "ok",
      });
    rs
}


fn f2(v: &serde_json::Value) -> serde_json::Value {
    println!("enter host f2, message: {}", v["message"]);
    println!("");
    let rs = json!({
        "message": "ok",
      });
    rs
}

fn f3(v: &serde_json::Value) -> serde_json::Value {
    println!("f3");
    println!("");
    serde_json::from_str("{}").unwrap()
}

// -----------------------------------------


#[derive(Default)]
pub struct MyImports;


// type FuncType = fn(&String)->&String;
lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, fn(&serde_json::Value)->serde_json::Value>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };

    static ref MODULE_FUNC: Mutex<HashMap<String, (Exports<Context<MyImports, ExportsData>>, Store<Context<MyImports, ExportsData>>)>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
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
    add_imports(&mut linker)?; //装载我们的实现到linker
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

    // for (key, value, _) in linker.iter(&mut store) {
    //     println!("{} / {}", key, value);

    // }

    Ok((exports, store))
}


fn registry(name: &str, f: fn(&serde_json::Value)->serde_json::Value) {
    {
        let mut map = HASHMAP.lock().unwrap();
        map.insert(String::from(name), f);
    }
}

fn registry_module(path: &str, name: &str) -> Result<()> {
    let (e, mut s) = instantiate(
        path,
        |linker| imports::add_to_linker(linker, |cx| -> &mut MyImports { &mut cx.imports }),
        |store, module, linker| Exports::instantiate(store, module, linker, |cx| &mut cx.exports),
    )?;
    {
        let mut map = MODULE_FUNC.lock().unwrap();
        map.insert(String::from(name), (e, s));
    }
    Ok(())
}

fn call_module_func(mname: &str, fname: &str, param: &serde_json::Value) -> serde_json::Value {

    let mut map = MODULE_FUNC.lock().unwrap();
    let (e, mut s) = map.remove(mname).unwrap();
    let rs = e.proxy(&mut s, fname, &param.to_string());      
    map.insert(String::from(mname), (e, s));
    // rs.unwrap()
    let v:serde_json::Value = serde_json::from_str(rs.unwrap().as_str()).unwrap();
    v
}

fn main() -> Result<()> {
    registry("f1", f1);
    registry("f2", f2);

    let john = json!({
      "name": "John Doe",
      "age": 43,
      "phones": [
          "+44 1234567",
          "+44 2345678"
      ]
    });

    impl Imports for MyImports {
        // 暴露给wasm module的函数

        fn proxy(&mut self, name: &str, param: &str) -> String {
            let mut map = HASHMAP.lock().unwrap();
            let param = String::from(param);
            let v:serde_json::Value = serde_json::from_str(&param).unwrap();
            let rs = map.get(name).unwrap()(&v);
            rs.to_string()
        }
    }

    // 加载 “module_A.wasm” ，并注册为 “module_A”
    registry_module("module_A.wasm", "module_A");
    call_module_func("module_A", "modulef1", &json!({"message": "call after first load",}));

    // 加载 “module_B.wasm” ，并注册为 “module_A”
    registry_module("module_B.wasm", "module_A");
    call_module_func("module_A", "modulef1", &json!({"message": "call after second load",}));


    let rs = call_module_func("module_A", "modulef2", &john);


    Ok(())
}
