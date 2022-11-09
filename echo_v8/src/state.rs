use v8::{Context, Global, HandleScope, OwnedIsolate};

pub struct JsRuntimeState {
    context: Option<Global<Context>>,
}

impl JsRuntimeState {
    pub fn new(isolate: &mut OwnedIsolate) -> Self {
        let handle_scope = &mut HandleScope::new(isolate);
        let context = Context::new(handle_scope);
        let global = Global::new(handle_scope, context);
        JsRuntimeState {
            context: Some(global),
        }
    }

    pub fn get_context(isolate: &mut OwnedIsolate) -> Global<Context> {
        isolate.get_slot::<Global<Context>>().unwrap().clone()
    }
}
