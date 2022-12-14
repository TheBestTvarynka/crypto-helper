use js_sys::Function;
use uuid::Uuid;
use wasm_bindgen::JsValue;
use yew::{classes, html, Callback, Html};

use crate::{
    crypto_helper::algorithm::KrbInput,
    notification::{Notification, NotificationType},
    utils::gen_copy_func,
};

const HMAC_LEN: usize = 12;

pub fn build_krb_output(
    krb_input: &KrbInput,
    output: &[u8],
    add_notification: Callback<Notification>,
) -> Html {
    let len = output.len();

    let (cipher_len, hmac_len, cipher, hmac) = if len < HMAC_LEN {
        (len, 0, hex::encode(output), "".into())
    } else {
        let cipher_len = len - HMAC_LEN;
        (
            cipher_len,
            HMAC_LEN,
            hex::encode(&output[0..cipher_len]),
            hex::encode(&output[cipher_len..]),
        )
    };

    let hex_output = hex::encode(output);
    let onclick = Callback::from(move |_| {
        let function = Function::new_no_args(&gen_copy_func(&hex_output));
        if function.call0(&JsValue::null()).is_ok() {
            add_notification.emit(Notification {
                id: Uuid::new_v4(),
                notification_type: NotificationType::Info,
                text: "Output copied".into(),
            })
        }
    });

    html! {
        <div class={classes!("output")}>
            <span class={classes!("full-cipher")} {onclick}>
                <span class={classes!("cipher")}>{cipher}</span>
                <span class={classes!("hmac")}>{hmac}</span>
            </span>
            {
                if krb_input.mode {
                    html!{ <span class={classes!("total")}>{format!("total: {}.", len)}</span> }
                } else {
                    html!{
                        <span class={classes!("total")}>
                            {format!("total: {}. cipher: {}. hmac: {}.", len, cipher_len, hmac_len)}
                        </span>
                    }
                }
            }
        </div>
    }
}
