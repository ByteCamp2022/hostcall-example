use std::{collections::HashMap, hash::Hash};
use lazy_static::*;
use std::sync::Mutex;
// use anyhow::Context;
use anyhow::Result;
use wasmtime::{*};

// wit_bindgen_wasmtime::export!("../imports.wit");
#[allow(clippy::all)]
pub mod imports {
  #[allow(unused_imports)]
  use wit_bindgen_wasmtime::{wasmtime, anyhow};
  pub trait Imports: Sized {
    fn proxy(&mut self,name: & str,param: & str,) -> String;
    
  }
  
  pub fn add_to_linker<T, U>(linker: &mut wasmtime::Linker<T>, get: impl Fn(&mut T) -> &mut U+ Send + Sync + Copy + 'static) -> anyhow::Result<()> 
  where U: Imports
  {
    use wit_bindgen_wasmtime::rt::get_memory;
    use wit_bindgen_wasmtime::rt::get_func;
    linker.func_wrap("imports", "proxy", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32,arg2:i32,arg3:i32,arg4:i32| {
      
      let func = get_func(&mut caller, "canonical_abi_realloc")?;
      let func_canonical_abi_realloc = func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
      let memory = &get_memory(&mut caller, "memory")?;
      let (mem, data) = memory.data_and_store_mut(&mut caller);
      let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
      let host = get(data);
      let ptr0 = arg0;
      let len0 = arg1;
      let ptr1 = arg2;
      let len1 = arg3;
      let param0 = _bc.slice_str(ptr0, len0)?;
      let param1 = _bc.slice_str(ptr1, len1)?;
      let result = host.proxy(param0, param1, );
      let vec2 = result;
      let ptr2 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, vec2.len() as i32))?;
      let caller_memory = memory.data_mut(&mut caller);
      caller_memory.store_many(ptr2, vec2.as_bytes())?;
      caller_memory.store(arg4 + 4, wit_bindgen_wasmtime::rt::as_i32(vec2.len() as i32))?;
      caller_memory.store(arg4 + 0, wit_bindgen_wasmtime::rt::as_i32(ptr2))?;
      Ok(())
    })?;
    Ok(())
  }
  use wit_bindgen_wasmtime::rt::RawMem;
}









// wit_bindgen_wasmtime::import!("../exports.wit");
#[allow(clippy::all)]
pub mod exports {
  #[allow(unused_imports)]
  use wit_bindgen_wasmtime::{wasmtime, anyhow};
  
  /// Auxiliary data associated with the wasm exports.
  ///
  /// This is required to be stored within the data of a
  /// `Store<T>` itself so lifting/lowering state can be managed
  /// when translating between the host and wasm.
  #[derive(Default)]
  pub struct ExportsData {
  }
  pub struct Exports<T> {
    get_state: Box<dyn Fn(&mut T) -> &mut ExportsData + Send + Sync>,
    canonical_abi_free: wasmtime::TypedFunc<(i32, i32, i32), ()>,
    canonical_abi_realloc: wasmtime::TypedFunc<(i32, i32, i32, i32), i32>,
    memory: wasmtime::Memory,
    proxy: wasmtime::TypedFunc<(i32,i32,i32,i32,), (i32,)>,
  }
  impl<T> Exports<T> {
    #[allow(unused_variables)]
    
    /// Adds any intrinsics, if necessary for this exported wasm
    /// functionality to the `linker` provided.
    ///
    /// The `get_state` closure is required to access the
    /// auxiliary data necessary for these wasm exports from
    /// the general store's state.
    pub fn add_to_linker(
    linker: &mut wasmtime::Linker<T>,
    get_state: impl Fn(&mut T) -> &mut ExportsData + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()> {
      Ok(())
    }
    
    /// Instantiates the provided `module` using the specified
    /// parameters, wrapping up the result in a structure that
    /// translates between wasm and the host.
    ///
    /// The `linker` provided will have intrinsics added to it
    /// automatically, so it's not necessary to call
    /// `add_to_linker` beforehand. This function will
    /// instantiate the `module` otherwise using `linker`, and
    /// both an instance of this structure and the underlying
    /// `wasmtime::Instance` will be returned.
    ///
    /// The `get_state` parameter is used to access the
    /// auxiliary state necessary for these wasm exports from
    /// the general store state `T`.
    pub fn instantiate(
    mut store: impl wasmtime::AsContextMut<Data = T>,
    module: &wasmtime::Module,
    linker: &mut wasmtime::Linker<T>,
    get_state: impl Fn(&mut T) -> &mut ExportsData + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<(Self, wasmtime::Instance)> {
      Self::add_to_linker(linker, get_state)?;
      let instance = linker.instantiate(&mut store, module)?;
      Ok((Self::new(store, &instance,get_state)?, instance))
    }
    
    /// Low-level creation wrapper for wrapping up the exports
    /// of the `instance` provided in this structure of wasm
    /// exports.
    ///
    /// This function will extract exports from the `instance`
    /// defined within `store` and wrap them all up in the
    /// returned structure which can be used to interact with
    /// the wasm module.
    pub fn new(
    mut store: impl wasmtime::AsContextMut<Data = T>,
    instance: &wasmtime::Instance,
    get_state: impl Fn(&mut T) -> &mut ExportsData + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<Self> {
      let mut store = store.as_context_mut();
      let canonical_abi_free= instance.get_typed_func::<(i32, i32, i32), (), _>(&mut store, "canonical_abi_free")?;
      let canonical_abi_realloc= instance.get_typed_func::<(i32, i32, i32, i32), i32, _>(&mut store, "canonical_abi_realloc")?;
      let memory= instance
      .get_memory(&mut store, "memory")
      .ok_or_else(|| {
        anyhow::anyhow!("`memory` export not a memory")
      })?
      ;
      let proxy= instance.get_typed_func::<(i32,i32,i32,i32,), (i32,), _>(&mut store, "proxy")?;
      Ok(Exports{
        canonical_abi_free,
        canonical_abi_realloc,
        memory,
        proxy,
        get_state: Box::new(get_state),
        
      })
    }
    pub fn proxy(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,name: & str,param: & str,)-> Result<String, wasmtime::Trap> {
      let func_canonical_abi_realloc = &self.canonical_abi_realloc;
      let func_canonical_abi_free = &self.canonical_abi_free;
      let memory = &self.memory;
      let vec0 = name;
      let ptr0 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, vec0.len() as i32))?;
      memory.data_mut(&mut caller).store_many(ptr0, vec0.as_bytes())?;
      let vec1 = param;
      let ptr1 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, vec1.len() as i32))?;
      memory.data_mut(&mut caller).store_many(ptr1, vec1.as_bytes())?;
      let (result2_0,) = self.proxy.call(&mut caller, (ptr0, vec0.len() as i32, ptr1, vec1.len() as i32, ))?;
      let load3 = memory.data_mut(&mut caller).load::<i32>(result2_0 + 0)?;
      let load4 = memory.data_mut(&mut caller).load::<i32>(result2_0 + 4)?;
      let ptr5 = load3;
      let len5 = load4;
      
      let data5 = copy_slice(
      &mut caller,
      memory,
      ptr5, len5, 1,
      )?;
      func_canonical_abi_free.call(&mut caller, (ptr5, len5, 1))?;
      Ok(String::from_utf8(data5)
      .map_err(|_| wasmtime::Trap::new("invalid utf-8"))?)
    }
  }
  use wit_bindgen_wasmtime::rt::RawMem;
  use wit_bindgen_wasmtime::rt::copy_slice;
}

use imports::*;
use exports::*;


// ----------------- 实际实现 --------------
fn f1(s: &String) -> &String {
    println!("from old f1, {}", s);
    s
}

fn f11(n: i32) -> i32 {
    println!("from new f1");
    n
}

fn f2(s: &String) -> &String {
    println!("from f2, {}", s);
    s
}

fn f3() {
    println!("f3");
}

// -----------------------------------------


#[derive(Default)]
pub struct MyImports;


// type FuncType = fn(&String)->&String;
lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, fn(&String)->&String>> = {
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


fn registry(name: &str, f: fn(&String)->&String) {
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

fn test(exp: Exports<Context<MyImports, ExportsData>>, store: Store<Context<MyImports, ExportsData>>) {
    exp.proxy(store, "f1", "sd");
}

fn main() -> Result<()> {
    registry("f1", f1);
    registry("f2", f2);

    impl Imports for MyImports {
        // 暴露给wasm module的函数

        fn proxy(&mut self, name: &str, param: &str) -> String {
            let mut map = HASHMAP.lock().unwrap();
            let param = String::from(param);

            let rs = map.get(name).unwrap()(&param);
            "sd".into()
        }

    }

    // 加载 “module_A.wasm” ，并注册为 “module_A”
    registry_module("module_A.wasm", "module_A");

    {
        let mut map = MODULE_FUNC.lock().unwrap();
        let (e, mut s) = map.remove("module_A").unwrap();
        e.proxy(&mut s, "modulef1", "vvvv");      
        map.insert(String::from("module_A"), (e, s));
    }

    // 加载 “module_B.wasm” ，并注册为 “module_A”
    registry_module("module_B.wasm", "module_A");

    {
        let mut map = MODULE_FUNC.lock().unwrap();
        let (e, mut s) = map.remove("module_A").unwrap();
        e.proxy(&mut s, "modulef1", "vvvv");   
        map.insert(String::from("module_A"), (e, s));
    }


    Ok(())
}
