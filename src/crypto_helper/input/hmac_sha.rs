use web_sys::{Event, HtmlInputElement};
use yew::{function_component, html, Callback, Html, Properties, TargetCast};

use crate::common::{build_byte_input, Switch};
use crate::crypto_helper::algorithm::{HmacShaAction, HmacShaInput as HmacShaInputData, HMAC_HASH_ALGOS};

#[derive(PartialEq, Properties, Clone)]
pub struct HmacShaInputProps {
    pub input: HmacShaInputData,
    pub input_setter: Callback<HmacShaInputData>,
}

#[function_component(HmacShaInput)]
fn hmac_sha_input(props: &HmacShaInputProps) -> Html {
    let HmacShaInputData {
        hash_alg,
        key,
        msg,
        action,
    } = props.input.clone();

    let input_setter = props.input_setter.clone();
    let input = props.input.clone();
    let on_switch = Callback::from(move |mode: bool| {
        let HmacShaInputData {
            hash_alg,
            key,
            msg,
            action: _,
        } = input.clone();
        input_setter.emit(HmacShaInputData {
            hash_alg,
            key,
            msg,
            action: mode.into(),
        });
    });

    let input_setter = props.input_setter.clone();
    let input = props.input.clone();
    let on_key_input = Callback::from(move |key| {
        let HmacShaInputData {
            hash_alg,
            key: _,
            msg,
            action,
        } = input.clone();
        input_setter.emit(HmacShaInputData {
            hash_alg,
            key,
            msg,
            action,
        });
    });

    let input_setter = props.input_setter.clone();
    let input = props.input.clone();
    let on_msg_input = Callback::from(move |msg| {
        let HmacShaInputData {
            hash_alg,
            key,
            msg: _,
            action,
        } = input.clone();
        input_setter.emit(HmacShaInputData {
            hash_alg,
            key,
            msg,
            action,
        });
    });

    let input_setter = props.input_setter.clone();
    let input = props.input.clone();
    let on_digest_input = Callback::from(move |digest| {
        let HmacShaInputData {
            hash_alg,
            key,
            msg,
            action: _,
        } = input.clone();
        input_setter.emit(HmacShaInputData {
            hash_alg,
            key,
            msg,
            action: HmacShaAction::Verify(digest),
        });
    });

    let input_setter = props.input_setter.clone();
    let input = props.input.clone();
    let onchange = Callback::from(move |event: Event| {
        let hash_alg: HtmlInputElement = event.target_unchecked_into();

        if let Ok(hash_alg) = hash_alg.value().as_str().try_into() {
            let HmacShaInputData {
                hash_alg: _,
                key,
                msg,
                action,
            } = input.clone();
            input_setter.emit(HmacShaInputData {
                hash_alg,
                key,
                msg,
                action,
            });
        }
    });

    html! {
        <div class="wide-input vertical">
            {build_byte_input(key, on_key_input, None, Some("key".into()))}
            {build_byte_input(msg, on_msg_input, None, Some("payload (message)".into()))}
            <div class="vertical">
                <div class="horizontal">
                    <select class="base-input auto-width-input" {onchange}>
                        {HMAC_HASH_ALGOS
                            .iter()
                            .map(|hash_algo_name| {
                                html! {
                                    <option
                                        selected={hash_alg == *hash_algo_name}
                                        value={hash_algo_name.to_string()}
                                    >
                                        {hash_algo_name}
                                    </option>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </select>
                    <div class="horizontal">
                        <span class="total">{"sign"}</span>
                        <Switch id={"hmac-verify".to_string()} setter={on_switch} state={bool::from(&action)}/>
                        <span class="total">{"verify"}</span>
                    </div>
                </div>
                {if let HmacShaAction::Verify(digest) = action {
                    build_byte_input(digest, on_digest_input, None, Some("digest".into()))
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}

pub fn build_hmac_sha_input(input: HmacShaInputData, input_setter: Callback<HmacShaInputData>) -> Html {
    html! {
        <HmacShaInput {input} {input_setter} />
    }
}
