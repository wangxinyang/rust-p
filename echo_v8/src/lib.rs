mod state;

use state::JsRuntimeState;
use v8::{CreateParams, Isolate, OwnedIsolate, V8};

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
        &self,
        source: impl AsRef<str>,
    ) -> Result<serde_json::Value, serde_json::Value> {
        todo!()
    }

    fn init_isolate(mut isolate: OwnedIsolate) -> Self {
        let state = JsRuntimeState::new(&mut isolate);
        isolate.set_slot(state);
        JsRuntime { isolate }
    }
}
