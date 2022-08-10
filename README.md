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
module a, message: call after first load
from f1, implemented in host
module b, message: call after seconed load
from f1, implemented in host
```
