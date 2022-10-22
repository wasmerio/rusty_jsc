use anyhow::{Context, Result};
use rusty_jsc::{JSContext, JSValue};
use rusty_jsc_macros::callback;
use std::fs;
use std::path::PathBuf;

pub fn run(input: PathBuf) -> Result<()> {
    let script = fs::read_to_string(&input)
        .with_context(|| format!("Failed to load module `{}`", input.display()))?;
    let mut context = JSContext::new();
    setup_prelude(&context);
    let _ = context.evaluate_script(&script, 1);
    if let Some(ex) = context.get_exception() {
        anyhow::bail!("Uncaught {}", ex.to_string(&context));
    }
    Ok(())
}

fn setup_prelude(context: &JSContext) {
    let require_fn = JSValue::callback(&context, Some(require));
    // require()
    let mut global = context.get_global_object();
    global.set_property(&context, "require".to_string(), require_fn);
    // foo()
    let callback = JSValue::callback(&context, Some(foo));
    global.set_property(&context, "foo".to_string(), callback);
}

#[callback]
fn require(_context: JSContext) {
    println!("warning: `require` is not implemented.")
}

#[callback]
fn foo(_context: JSContext) {
    println!("hello from Rust land!");
}
