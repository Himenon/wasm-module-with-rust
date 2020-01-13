import("./lib/wasm_module_with_rust").then(module => {
  console.log(module.hello_world());
});
