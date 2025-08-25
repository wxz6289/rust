use std::error::Error;
use wasmtime::*;

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "example/hello.wasm")?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    let answer = instance
        .get_func(&mut store, "answer").expect("answer function not found");
    let answer = answer.typed::<(), i32>(&store)?;
    let result = answer.call(&mut store, ())?;
    println!("The answer is: {}", result);
    Ok(())
}
