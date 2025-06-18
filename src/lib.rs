use rdev::{listen, Event, EventType}; 
use napi::{Env, JsFunction, Result};
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi_derive::napi;
use std::thread;

#[napi]
pub fn start(callback: JsFunction) -> Result<()> {
    println!("[rustiohook] Listener started. Awaiting key presses and clicks...");

    let tsfn: ThreadsafeFunction<Event> = callback
        .create_threadsafe_function(0, |ctx| {
            let event: &Event = &ctx.value;
            
            // Фильтруем "шумные" события, чтобы они не печатались
            match event.event_type {
                EventType::MouseMove { .. } | EventType::Wheel { .. } => {
                    // Ничего не делаем и не печатаем
                },
                // Для всех остальных, "важных" событий...
                _ => {
                    // ...проверяем, есть ли у события имя (символ)
                    print!("[RUST HANDLED]: "); // печатаем префикс без переноса строки
                    
                    match &event.name {
                        // Если есть символ (Some(name)), печатаем его вместе с типом события
                        Some(name) => {
                            println!("{:?} (Символ: '{}')", event.event_type, name);
                        },
                        // Если символа нет (None), просто печатаем тип события
                        None => {
                            println!("{:?}", event.event_type);
                        }
                    }
                }
            }

            // Передаем полное событие в JavaScript, как и раньше
            let event_str = format!("{:?}", event);
            let env = ctx.env;
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