use js_sys::Function;
use wasm_bindgen::JsValue;
use yew::{
    classes, function_component, html, use_effect_with_deps, use_state, Callback, Html, Properties,
};
use yew_notifications::{Notification, NotificationType};

use crate::{utils::gen_copy_func, common::{BYTES_FORMATS, encode_bytes, get_format_button_class, get_set_format_callback}};

use super::BytesFormat;

#[derive(PartialEq, Properties, Clone)]
pub struct SimpleOutputProps {
    output: Vec<u8>,
    format: BytesFormat,
    add_notification: Callback<Notification>,
}

#[function_component(SimpleOutput)]
pub fn simple_output(props: &SimpleOutputProps) -> Html {
    let SimpleOutputProps {
        output,
        format,
        add_notification,
    } = props.clone();

    let bytes_format = use_state(|| format);

    let format_setter = bytes_format.setter();
    use_effect_with_deps(
        move |format| {
            format_setter.set(**format);
        },
        bytes_format.clone(),
    );

    let encoded_bytes = encode_bytes(&output, &bytes_format);

    let encoded = encoded_bytes.clone();
    let onclick = Callback::from(move |_| {
        let function = Function::new_no_args(&gen_copy_func(&encoded));
        if function.call0(&JsValue::null()).is_ok() {
            add_notification.emit(Notification::from_description_and_type(
                NotificationType::Info,
                "output copied",
            ))
        }
    });

    html! {
        <div class={classes!("output")}>
            <div class={classes!("formats-container")}>{
                BYTES_FORMATS.iter().map(|format| {
                    html! {
                        <button
                            class={get_format_button_class(*bytes_format == *format)}
                            onclick={get_set_format_callback(*format, bytes_format.setter())}
                        >
                            {<&str>::from(format)}
                        </button>
                    }
                }).collect::<Html>()
            }</div>
            <span class={classes!("simple-digest")} onclick={onclick}>{encoded_bytes}</span>
            <span class={classes!("total")}>{format!("total: {}", output.len())}</span>
        </div>
    }
}

pub fn build_simple_output(output: Vec<u8>, format: BytesFormat, add_notification: Callback<Notification>) -> Html {
    html! {
        <SimpleOutput {output} {format} {add_notification} />
    }
}
