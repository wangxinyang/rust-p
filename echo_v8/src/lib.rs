mod state;

use state::JsRuntimeState;
use v8::{CreateParams, HandleScope, Isolate, Local, OwnedIsolate, TryCatch, Value, V8};

#[derive(Default)]
pub struct JsRuntimeParams {
    pub params: CreateParams,
}

impl JsRuntimeParams {
    fn into_inner(self) -> CreateParams {
        self.params
    }
}

pub struct JsRuntime {
    isolate: OwnedIsolate,
}

impl JsRuntime {
    pub fn init() {
        let platform = v8::new_default_platform(0, false).make_shared();
        V8::initialize_platform(platform);
        V8::initialize();
    }

    pub fn new(params: JsRuntimeParams) -> Self {
        let isolate = Isolate::new(params.into_inner());
        JsRuntime::init_isolate(isolate)
    }

    pub fn execute_script(
        &mut self,
        source: impl AsRef<str>,
    ) -> Result<serde_json::Value, serde_json::Value> {
        let context = JsRuntimeState::get_context(&mut self.isolate);
        let handle_scope = &mut HandleScope::with_context(&mut self.isolate, context);
        match execute_script(handle_scope, source) {
            Ok(input) => Ok(serde_v8::from_v8(handle_scope, input).unwrap()),
            Err(err) => Err(serde_v8::from_v8(handle_scope, err).unwrap()),
        }
    }

    fn init_isolate(mut isolate: OwnedIsolate) -> Self {
        let state = JsRuntimeState::new(&mut isolate);
        isolate.set_slot(state);
        JsRuntime { isolate }
    }
}

fn execute_script<'a>(
    scope: &mut HandleScope<'a>,
    code: impl AsRef<str>,
) -> Result<Local<'a, Value>, Local<'a, Value>> {
    let scope = &mut TryCatch::new(scope);
    let source = v8::String::new(scope, code.as_ref()).unwrap();
    println!("source {:?}", source);
    let res = v8::Script::compile(scope, source, None);
    println!("scope {:?}", scope);
    println!("res {:?}", res);
    let result = res.unwrap().run(scope).unwrap();
    // .and_then(|script| script.run(scope))

    Ok(result)
}
