use deno_core::{
    anyhow::{Ok, Result},
    JsRuntime, RuntimeOptions,
};

#[tokio::main]
async fn main() -> Result<()> {
    let runtime_options = RuntimeOptions::default();
    let mut runtime = JsRuntime::new(runtime_options);
    let code = include_str!("basic.js");
    runtime.execute_script("anon", code)?;
    Ok(())
}
