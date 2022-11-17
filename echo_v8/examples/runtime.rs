use echo_v8::{JsRuntime, JsRuntimeParams};

fn main() {
    JsRuntime::init();
    let params = JsRuntimeParams::default();
    let mut runtime = JsRuntime::new(params);

    let code = r#"
        fn hello() {
            console.log("Hello World!");
        }
        hello();
    "#;

    let result = runtime.execute_script(code);
    println!("result: {:?}", result);
}
