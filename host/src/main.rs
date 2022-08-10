// use anyhow::Context;
use anyhow::Result;
use wasmtime::*;

wit_bindgen_wasmtime::export!("../imports.wit");
// #[allow(clippy::all)]
// pub mod imports {
//   #[allow(unused_imports)]
//   use wit_bindgen_wasmtime::{wasmtime, anyhow};
//   pub trait Imports: Sized {
//     fn hostf1(&mut self,s: & str,) -> ();
    
//   }
  
//   pub fn add_to_linker<T, U>(linker: &mut wasmtime::Linker<T>, get: impl Fn(&mut T) -> &mut U+ Send + Sync + Copy + 'static) -> anyhow::Result<()> 
//   where U: Imports
//   {
//     use wit_bindgen_wasmtime::rt::get_memory;
//     linker.func_wrap("imports", "hostf1", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32| {
//       let memory = &get_memory(&mut caller, "memory")?;
//       let (mem, data) = memory.data_and_store_mut(&mut caller);
//       let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
//       let host = get(data);
//       let ptr0 = arg0;
//       let len0 = arg1;
//       let param0 = _bc.slice_str(ptr0, len0)?;
//       let result = host.hostf1(param0, );
//       let () = result;
//       Ok(())
//     })?;
//     Ok(())
//   }
// }



wit_bindgen_wasmtime::import!("../exports.wit");
// #[allow(clippy::all)]
// pub mod exports {
//   #[allow(unused_imports)]
//   use wit_bindgen_wasmtime::{wasmtime, anyhow};
  
//   /// Auxiliary data associated with the wasm exports.
//   ///
//   /// This is required to be stored within the data of a
//   /// `Store<T>` itself so lifting/lowering state can be managed
//   /// when translating between the host and wasm.
//   #[derive(Default)]
//   pub struct ExportsData {
//   }
//   pub struct Exports<T> {
//     get_state: Box<dyn Fn(&mut T) -> &mut ExportsData + Send + Sync>,
//     canonical_abi_free: wasmtime::TypedFunc<(i32, i32, i32), ()>,
//     canonical_abi_realloc: wasmtime::TypedFunc<(i32, i32, i32, i32), i32>,
//     memory: wasmtime::Memory,
//     modulef1: wasmtime::TypedFunc<(i32,i32,), ()>,
//     modulef2: wasmtime::TypedFunc<(), (i32,)>,
//     modulef3: wasmtime::TypedFunc<(), ()>,
//     modulef4: wasmtime::TypedFunc<(i32,i32,), ()>,
//   }
//   impl<T> Exports<T> {
//     #[allow(unused_variables)]
    
//     /// Adds any intrinsics, if necessary for this exported wasm
//     /// functionality to the `linker` provided.
//     ///
//     /// The `get_state` closure is required to access the
//     /// auxiliary data necessary for these wasm exports from
//     /// the general store's state.
//     pub fn add_to_linker(
//     linker: &mut wasmtime::Linker<T>,
//     get_state: impl Fn(&mut T) -> &mut ExportsData + Send + Sync + Copy + 'static,
//     ) -> anyhow::Result<()> {
//       Ok(())
//     }
    
//     /// Instantiates the provided `module` using the specified
//     /// parameters, wrapping up the result in a structure that
//     /// translates between wasm and the host.
//     ///
//     /// The `linker` provided will have intrinsics added to it
//     /// automatically, so it's not necessary to call
//     /// `add_to_linker` beforehand. This function will
//     /// instantiate the `module` otherwise using `linker`, and
//     /// both an instance of this structure and the underlying
//     /// `wasmtime::Instance` will be returned.
//     ///
//     /// The `get_state` parameter is used to access the
//     /// auxiliary state necessary for these wasm exports from
//     /// the general store state `T`.
//     pub fn instantiate(
//     mut store: impl wasmtime::AsContextMut<Data = T>,
//     module: &wasmtime::Module,
//     linker: &mut wasmtime::Linker<T>,
//     get_state: impl Fn(&mut T) -> &mut ExportsData + Send + Sync + Copy + 'static,
//     ) -> anyhow::Result<(Self, wasmtime::Instance)> {
//       Self::add_to_linker(linker, get_state)?;
//       let instance = linker.instantiate(&mut store, module)?;
//       Ok((Self::new(store, &instance,get_state)?, instance))
//     }
    
//     /// Low-level creation wrapper for wrapping up the exports
//     /// of the `instance` provided in this structure of wasm
//     /// exports.
//     ///
//     /// This function will extract exports from the `instance`
//     /// defined within `store` and wrap them all up in the
//     /// returned structure which can be used to interact with
//     /// the wasm module.
//     pub fn new(
//     mut store: impl wasmtime::AsContextMut<Data = T>,
//     instance: &wasmtime::Instance,
//     get_state: impl Fn(&mut T) -> &mut ExportsData + Send + Sync + Copy + 'static,
//     ) -> anyhow::Result<Self> {
//       let mut store = store.as_context_mut();
//       let canonical_abi_free= instance.get_typed_func::<(i32, i32, i32), (), _>(&mut store, "canonical_abi_free")?;
//       let canonical_abi_realloc= instance.get_typed_func::<(i32, i32, i32, i32), i32, _>(&mut store, "canonical_abi_realloc")?;
//       let memory= instance
//       .get_memory(&mut store, "memory")
//       .ok_or_else(|| {
//         anyhow::anyhow!("`memory` export not a memory")
//       })?
//       ;
//       let modulef1= instance.get_typed_func::<(i32,i32,), (), _>(&mut store, "modulef1")?;
//       let modulef2= instance.get_typed_func::<(), (i32,), _>(&mut store, "modulef2")?;
//       let modulef3= instance.get_typed_func::<(), (), _>(&mut store, "modulef3")?;
//       let modulef4= instance.get_typed_func::<(i32,i32,), (), _>(&mut store, "modulef4")?;
//       Ok(Exports{
//         canonical_abi_free,
//         canonical_abi_realloc,
//         memory,
//         modulef1,
//         modulef2,
//         modulef3,
//         modulef4,
//         get_state: Box::new(get_state),
        
//       })
//     }
//     pub fn modulef1(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,s: & str,)-> Result<(), wasmtime::Trap> {
//       let func_canonical_abi_realloc = &self.canonical_abi_realloc;
//       let memory = &self.memory;
//       let vec0 = s;
//       let ptr0 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, vec0.len() as i32))?;
//       memory.data_mut(&mut caller).store_many(ptr0, vec0.as_bytes())?;
//       self.modulef1.call(&mut caller, (ptr0, vec0.len() as i32, ))?;
//       Ok(())
//     }
//     pub fn modulef2(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,)-> Result<String, wasmtime::Trap> {
//       let func_canonical_abi_free = &self.canonical_abi_free;
//       let memory = &self.memory;
//       let (result0_0,) = self.modulef2.call(&mut caller, ())?;
//       let load1 = memory.data_mut(&mut caller).load::<i32>(result0_0 + 0)?;
//       let load2 = memory.data_mut(&mut caller).load::<i32>(result0_0 + 4)?;
//       let ptr3 = load1;
//       let len3 = load2;
      
//       let data3 = copy_slice(
//       &mut caller,
//       memory,
//       ptr3, len3, 1,
//       )?;
//       func_canonical_abi_free.call(&mut caller, (ptr3, len3, 1))?;
//       Ok(String::from_utf8(data3)
//       .map_err(|_| wasmtime::Trap::new("invalid utf-8"))?)
//     }
//     pub fn modulef3(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,)-> Result<(), wasmtime::Trap> {
//       self.modulef3.call(&mut caller, ())?;
//       Ok(())
//     }
//     pub fn modulef4(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,ls: &[u8],)-> Result<(), wasmtime::Trap> {
//       let func_canonical_abi_realloc = &self.canonical_abi_realloc;
//       let memory = &self.memory;
//       let vec0 = ls;
//       let ptr0 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec0.len() as i32) * 1))?;
//       memory.data_mut(&mut caller).store_many(ptr0, &vec0)?;
//       self.modulef4.call(&mut caller, (ptr0, vec0.len() as i32, ))?;
//       Ok(())
//     }
//   }
//   use wit_bindgen_wasmtime::rt::RawMem;
//   use wit_bindgen_wasmtime::rt::copy_slice;
// }

use imports::*;
use exports::*;


// ----------------- 实际实现 --------------
fn f1(n: i32) -> i32 {
    println!("from old f1");
    n
}

fn f11(n: i32) -> i32 {
    println!("from new f1");
    n
}

fn f2(s: &str) -> &str {
    println!("f2");
    s
}

fn f3() {
    println!("f3");
}

// -----------------------------------------

struct FuncMap {
    f1: fn(i32) -> i32,
    f2: fn(&str) -> &str,
}

// 全局变量
static mut fm: FuncMap = FuncMap {
    f1: f1,
    f2: f2,
};


#[derive(Default)]
pub struct MyImports;


impl Imports for MyImports {
    // 暴露给wasm module的函数
    fn hostf1(&mut self, s: &str) {
        // println!("{}", s);
        unsafe {
            // 调用实际实现
            (fm.f1)(5);
        }
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





fn main() -> Result<()> {

    let (exports, mut store) = crate::instantiate(
        "module_A.wasm",
        |linker| imports::add_to_linker(linker, |cx| -> &mut MyImports { &mut cx.imports }),
        |store, module, linker| Exports::instantiate(store, module, linker, |cx| &mut cx.exports),
    )?;
    exports.modulef1(&mut store, "sdf")?;

    unsafe {
        fm.f1 = f11;
    }

    exports.modulef1(&mut store, "sdf")?;
    // let mut v = [3, 2, 3, 4];
    // exports.modulef4(&mut store, &v)?;
    
    Ok(())
}
