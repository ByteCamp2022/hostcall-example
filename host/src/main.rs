// use anyhow::Context;
use anyhow::Result;
use wasmtime::*;

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
