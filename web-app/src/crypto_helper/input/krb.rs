use picky_krb::crypto::CipherSuite;
use web_sys::{HtmlInputElement, InputEvent, MouseEvent};
use yew::{
    classes, function_component, html, use_effect, use_state, Callback, Html, Properties,
    TargetCast, UseStateSetter,
};

use crate::common::Switch;
use crate::crypto_helper::algorithm::KrbInput as KerberosInput;

fn get_usage_number_name(usage_number: &str) -> &str {
    match usage_number.parse::<u32>() {
        Ok(number) => match number {
            1 => "AS-REQ PA-ENC-TIMESTAMP",
            2 => "AS-REP Ticket",
            3 => "AS-REP Enc part",
            4 => "TGS-REQ KDC-REQ-BODY AuthData (session key)",
            5 => "TGS-REQ KDC-REQ-BODY AuthData (authenticator subkey)",
            6 => "TGS-REQ PA-TGS-REQ padata AP-REQ Authenticator cksum (session key)",
            7 => "TGS-REQ PA-TGS-REQ padata AP-REQ Authenticator (session key)",
            8 => "TGS-REP enc part (session key)",
            9 => "TGS-REP enc part (authenticator subkey)",
            10 => "AP-REQ Authenticator cksum (session key)",
            11 => "AP-REQ Authenticator (session key)",
            12 => "AP-REP enc part (session key)",
            13 => "KRB-PRIV enc part",
            14 => "KRB-CRED enc part",
            15 => "KRB-SAFE cksum",
            19 => "AD-KDC-ISSUED cksum",
            22 => "KG-USAGE-ACCEPTOR-SEAL",
            23 => "KG-USAGE-ACCEPTOR-SIGN",
            24 => "KG-USAGE-INITIATOR-SEAL",
            25 => "KG-USAGE-INITIATOR-SIGN",
            41 => "PKU2U_KRB_FINISHED",
            _ => "?unknown?",
        },
        Err(_) => "?unknown?",
    }
}

fn gen_on_input_handle(setter: UseStateSetter<String>) -> Callback<InputEvent> {
    Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        let value = input.value();
        log::debug!("new value: {}", value);

        setter.set(value);
    })
}

fn generate_key(cipher: &CipherSuite, password: &str, salt: &str) -> String {
    hex::encode(
        cipher
            .cipher()
            .generate_key_from_password(password.as_bytes(), salt.as_bytes())
            .unwrap(),
    )
}

#[derive(Debug, Properties, PartialEq)]
pub struct KrbInputProps {
    pub krb_input: KerberosInput,
    pub krb_input_setter: Callback<KerberosInput>,
    pub krb_algo: CipherSuite,
}

#[function_component(KrbInput)]
pub fn krb_input(props: &KrbInputProps) -> Html {
    let key = use_state(String::new);
    let usage_number = use_state(String::new);
    let payload = use_state(String::new);

    // false - encrypt
    // true - decrypt
    let mode = use_state(|| false);

    // false - user provides the key by yourself
    // true - generate key from the password
    let key_source = use_state(|| false);
    let password = use_state(String::new);
    let salt = use_state(String::new);

    let mode_value = *mode;
    let key_value = (*key).clone();
    let usage_number_value = (*usage_number).clone();
    let payload_value = (*payload).clone();
    let input_setter = props.krb_input_setter.clone();
    use_effect(move || {
        log::debug!(
            "use_effect: {}, {}, {}, {}",
            mode_value,
            key_value,
            usage_number_value,
            payload_value
        );

        input_setter.emit(KerberosInput {
            mode: mode_value,
            key: key_value,
            key_usage: usage_number_value,
            payload: payload_value,
        });
    });

    let key_setter = key.setter();
    let cipher = props.krb_algo.clone();
    let password_value = (*password).clone();
    let salt_value = (*salt).clone();
    let generate_key_from_password = Callback::from(move |event: MouseEvent| {
        event.prevent_default();
        key_setter.set(generate_key(&cipher, &password_value, &salt_value))
    });

    html! {
        <div class={classes!("enc-params")}>
            <div class={classes!("vertical")}>
                <input
                    class={classes!("base-input")}
                    placeholder={"key"}
                    value={(*key).clone()}
                    oninput={gen_on_input_handle(key.setter())}
                />
                <span class={classes!("total")}>{"len: "}<span>{(*key).len() / 2}</span></span>
            </div>
            <div class={classes!("vertical")}>
                <input
                    type={"number"}
                    class={classes!("base-input")}
                    placeholder={"usage number"}
                    value={usage_number.to_string()}
                    oninput={gen_on_input_handle(usage_number.setter())}
                />
                <span class={classes!("total")}>{get_usage_number_name((*usage_number).as_str())}</span>
            </div>
            <div class={classes!("vertical")}>
                <input
                    class={classes!("base-input")}
                    placeholder={"payload"}
                    value={(*payload).clone()}
                    oninput={gen_on_input_handle(payload.setter())}
                />
                <span class={classes!("total")}>{"len: "}<span>{(*payload).len() / 2}</span></span>
            </div>
            <div class={classes!("horizontal", "krbEncOpts")}>
                <span class="total">{"encrypt"}</span>
                <Switch id={"1"} setter={mode.setter()} state={*mode} />
                <span class="total">{"decrypt"}</span>
                <span class="total">{"|"}</span>
                <span class="total">{"raw key"}</span>
                <Switch id={"2"} setter={key_source.setter()} state={*key_source} />
                <span class="total">{"key from password"}</span>
            </div>
            {
                if *key_source {
                    html!{
                        <form class={classes!("horizontal", "generateKeyOpts")}>
                            <div>
                                <input
                                    class={classes!("base-input")}
                                    placeholder={"password"}
                                    value={(*password).clone()}
                                    oninput={gen_on_input_handle(password.setter())}
                                />
                            </div>
                            <div>
                                <input
                                    class={classes!("base-input")}
                                    placeholder={"salt"}
                                    value={(*salt).clone()}
                                    oninput={gen_on_input_handle(salt.setter())}
                                />
                            </div>
                            <button type={"submit"} onclick={generate_key_from_password}>{"Generate key"}</button>
                        </form>
                    }
                } else {
                    html!{}
                }
            }
        </div>
    }
}

pub fn build_krb_input(
    krb_input: KerberosInput,
    krb_input_setter: Callback<KerberosInput>,
    krb_algo: CipherSuite,
) -> Html {
    html! {
        <KrbInput krb_input={krb_input} krb_input_setter={krb_input_setter} krb_algo={krb_algo} />
    }
}
