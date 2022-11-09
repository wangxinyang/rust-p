use serde::{Deserialize, Serialize};
use v8::{Context, ContextScope, CreateParams, HandleScope, Isolate, Script, V8};

#[derive(Debug, Serialize, Deserialize)]
struct Result {
    code: usize,
    message: String,
}

fn main() {
    // init v8
    init();

    // create a new isolate
    let isolate = &mut Isolate::new(CreateParams::default());
    // create a hanlde scope
    let handle_scope = &mut HandleScope::new(isolate);
    // create a new Context
    let context = Context::new(handle_scope);
    // create context scope
    let context_scope = &mut ContextScope::new(handle_scope, context);

    let source = r#"
        function hello() {
            return {
                code: 200,
                message: "execute success"
            };
        }
        hello();
    "#;
    let code = v8::String::new(context_scope, source).unwrap();
    let script = Script::compile(context_scope, code, None).unwrap();
    let result = script.run(context_scope).unwrap();
    let result: Result = serde_v8::from_v8(context_scope, result).unwrap();
    println!("Result is: {result:?}");
}

fn init() {
    let platform = v8::new_default_platform(0, false).make_shared();
    V8::initialize_platform(platform);
    V8::initialize();
}
