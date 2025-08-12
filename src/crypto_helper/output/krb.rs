use yew::{Callback, Html, Properties, function_component, html};
use yew_hooks::use_clipboard;
use yew_notifications::{Notification, NotificationType};

use crate::crypto_helper::algorithm::KrbMode;

const HMAC_LEN: usize = 12;

#[derive(PartialEq, Properties, Clone)]
pub struct KrbOutputProps {
    mode: KrbMode,
    output: Vec<u8>,
    add_notification: Callback<Notification>,
}

#[function_component(KrbOutput)]
pub fn krb_output(props: &KrbOutputProps) -> Html {
    let KrbOutputProps {
        mode,
        output,
        add_notification,
    } = &props;

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
    let clipboard = use_clipboard();
    let add_notification = add_notification.clone();
    let onclick = Callback::from(move |_| {
        clipboard.write_text(hex_output.clone());

        add_notification.emit(Notification::from_description_and_type(
            NotificationType::Info,
            "output copied",
        ));
    });

    html! {
        <div class="output">
            <span class="full-cipher" {onclick}>
                <span class="cipher">{cipher}</span>
                <span class="hmac">{hmac}</span>
            </span>
            {
                match mode {
                    KrbMode::Encrypt => html!{
                        <span class="total">
                            {format!("total: {}. cipher: {}. hmac: {}.", len, cipher_len, hmac_len)}
                        </span>
                    },
                    KrbMode::Decrypt => html!{
                        <span class="total">{format!("total: {}.", len)}</span>
                    }
                }
            }
        </div>
    }
}

pub fn build_krb_output(mode: KrbMode, output: Vec<u8>, add_notification: Callback<Notification>) -> Html {
    html! {
        <KrbOutput {mode} {output} {add_notification} />
    }
}
