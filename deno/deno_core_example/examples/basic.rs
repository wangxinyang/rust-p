use deno_core::{
    anyhow::{Ok, Result},
    v8::Local,
    JsRuntime, RuntimeOptions,
};

#[tokio::main]
async fn main() -> Result<()> {
    let runtime_options = RuntimeOptions::default();
    let mut runtime = JsRuntime::new(runtime_options);
    let code = include_str!("basic.js");
    let ret = runtime.execute_script("anon", code)?;
    let ret = runtime.resolve_value(ret).await?;
    let handle_scope = &mut runtime.handle_scope();
    let local_ret = Local::new(handle_scope, ret);
    // println!("Rust is promise : {}", local_ret.is_promise()); // not execute runtime.resolve_value(ret).await?; Rust is promise : true
    let value: String = serde_v8::from_v8(handle_scope, local_ret)?;
    println!("result: {:?}", value);
    Ok(())
}
