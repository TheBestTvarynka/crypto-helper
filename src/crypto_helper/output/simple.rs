use js_sys::Function;
use wasm_bindgen::JsValue;
use yew::{classes, html, Callback, Html};
use yew_notifications::{Notification, NotificationType};

use crate::utils::gen_copy_func;

pub fn build_simple_output(output: &[u8], add_notification: Callback<Notification>) -> Html {
    let hex_output = hex::encode(output);
    let onclick = Callback::from(move |_| {
        let function = Function::new_no_args(&gen_copy_func(&hex_output));
        if function.call0(&JsValue::null()).is_ok() {
            add_notification.emit(Notification::from_description_and_type(
                NotificationType::Info,
                "output copied",
            ))
        }
    });

    html! {
        <div class={classes!("output")} {onclick}>
            <span class={classes!("simple-digest")}>{hex::encode(output)}</span>
            <span class={classes!("total")}>{format!("total: {}", output.len())}</span>
        </div>
    }
}
