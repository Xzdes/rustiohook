use napi::{Env, JsFunction, Result}; // <-- Удалили JsString отсюда
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi_derive::napi;
use rdev::{listen, Event};
use std::thread;

#[napi]
pub fn start(callback: JsFunction) -> Result<()> {
    let tsfn: ThreadsafeFunction<Event> = callback
        .create_threadsafe_function(0, |ctx| {
            let env = ctx.env;
            let event = ctx.value;
            let event_str = format!("{:?}", event);
            println!("RUST EVENT: {}", event_str); // Оставляем эту критически важную строку
            let js_string = env.create_string(&event_str)?;
            Ok(vec![js_string])
        })?;

    thread::spawn(move || {
        let _ = listen(move |event| {
            let _ = tsfn.call(Ok(event), ThreadsafeFunctionCallMode::NonBlocking);
        });
    });
    Ok(())
}

#[napi]
pub fn stop(env: Env) -> Result<napi::JsUndefined> {
    println!("[rustiohook] Stop function called. Manual exit (Ctrl+C) is required for now.");
    env.get_undefined()
}