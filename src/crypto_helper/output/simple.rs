// use wasm_bindgen::{JsValue, prelude::Closure};
use yew::{classes, html, Callback, Html};

pub fn build_simple_output(output: &[u8]) -> Html {
    // let hex_output = hex::encode(output);
    let onclick = Callback::from(move |_| {
        // let window = web_sys::window().unwrap();
        // let navigator = window.navigator();
        // let clippboard = navigator.clipboard().unwrap();
        // clippboard.writeText("");
        // clippboard.write(&JsValue::from_str(&hex_output)).then(&Closure::new(|_| {
        //     log::info!("copied");
        // }));
    });

    html! {
        <div class={classes!("output")} onclick={onclick}>
            <span class={classes!("simple-digest")}>{hex::encode(output)}</span>
            <span class={classes!("total")}>{format!("total: {}", output.len())}</span>
        </div>
    }
}
