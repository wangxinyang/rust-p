use echo_v8::{JsRuntime, JsRuntimeParams};

fn main() {
    JsRuntime::init();

    let mut runtime = JsRuntime::new(JsRuntimeParams::default());

    let source = r#"
        function hello() {
            return {
                code: 200,
                message: "execute success"
            };
        }
        hello();
    "#;
    let _result = runtime.execute_script(source).unwrap();
}
