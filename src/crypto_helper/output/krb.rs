use yew::{classes, html, Html};

use crate::crypto_helper::algorithm::KrbInput;

const HMAC_LEN: usize = 12;

pub fn build_krb_output(krb_input: &KrbInput, output: &[u8]) -> Html {
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

    html! {
        <div class={classes!("output")}>
            <span class={classes!("full-cipher")}>
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
