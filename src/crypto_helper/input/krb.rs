use picky_krb::crypto::CipherSuite;
use web_sys::{HtmlInputElement, InputEvent, MouseEvent};
use yew::{
    classes, function_component, html, use_effect_with_deps, use_state, Callback, Html, Properties, TargetCast,
    UseStateSetter,
};
use yew_notifications::{use_notification, Notification, NotificationType};

use crate::common::{build_byte_input, Switch};
use crate::crypto_helper::algorithm::{KrbInput as KerberosInput, KrbMode};

fn get_usage_number_name(usage_number: i32) -> &'static str {
    match usage_number {
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
    }
}

fn gen_on_input_handle(setter: UseStateSetter<String>) -> Callback<InputEvent> {
    Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        setter.set(input.value());
    })
}

fn generate_key(cipher: &CipherSuite, password: &str, salt: &str) -> Result<Vec<u8>, String> {
    cipher
        .cipher()
        .generate_key_from_password(password.as_bytes(), salt.as_bytes())
        .map_err(|e| e.to_string())
}

#[derive(PartialEq, Properties, Clone)]
pub struct KrbInputProps {
    pub krb_input: KerberosInput,
    pub krb_input_setter: Callback<KerberosInput>,
    // needs it for the key generation algorithm
    pub krb_algo: CipherSuite,

    // options
    pub with_mode: bool,
}

#[function_component(KrbInput)]
pub fn krb_input(props: &KrbInputProps) -> Html {
    let krb_input = props.krb_input.clone();
    let krb_input = use_state(|| krb_input);

    let krb_input_setter = krb_input.setter();
    use_effect_with_deps(
        move |props| {
            krb_input_setter.set(props.krb_input.clone());
        },
        props.clone(),
    );

    // false - user provides the key by yourself
    // true - generate key from the password
    let key_source = use_state(|| false);
    let key_source_setter = key_source.setter();
    let set_key_source = Callback::from(move |key_source| {
        key_source_setter.set(key_source);
    });
    let password = use_state(String::new);
    let salt = use_state(String::new);

    let cipher = props.krb_algo.clone();
    let password_value = (*password).clone();
    let salt_value = (*salt).clone();

    let notifications = use_notification::<Notification>();
    let input_setter = props.krb_input_setter.clone();
    let mode = krb_input.mode;
    let krb_data = krb_input.data.clone();
    let generate_key_from_password = Callback::from(move |event: MouseEvent| {
        event.prevent_default();
        match generate_key(&cipher, &password_value, &salt_value) {
            Ok(key) => {
                let mut data = krb_data.clone();
                data.key = key;
                input_setter.emit(KerberosInput { mode, data });
            }
            Err(err) => notifications.spawn(Notification::new(
                NotificationType::Error,
                "KRB key generation",
                err,
                Notification::NOTIFICATION_LIFETIME,
            )),
        };
    });

    let input_setter = props.krb_input_setter.clone();
    let krb_data = krb_input.data.clone();
    let set_key = Callback::from(move |key| {
        let mut data = krb_data.clone();
        data.key = key;
        input_setter.emit(KerberosInput { mode, data });
    });

    let input_setter = props.krb_input_setter.clone();
    let krb_data = krb_input.data.clone();
    let set_mode = Callback::from(move |mode: bool| {
        input_setter.emit(KerberosInput {
            mode: mode.into(),
            data: krb_data.clone(),
        });
    });

    let input_setter = props.krb_input_setter.clone();
    let notifications = use_notification::<Notification>();
    let krb_data = krb_input.data.clone();
    let set_usage_number = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        match input.value().parse() {
            Ok(usage_number) => {
                let mut data = krb_data.clone();
                data.key_usage = usage_number;
                input_setter.emit(KerberosInput { mode, data });
            }
            Err(err) => notifications.spawn(Notification::new(
                NotificationType::Error,
                "KRB key usage parsing",
                err.to_string(),
                Notification::NOTIFICATION_LIFETIME,
            )),
        };
    });

    let input_setter = props.krb_input_setter.clone();
    let krb_data = krb_input.data.clone();
    let set_payload = Callback::from(move |payload| {
        let mut data = krb_data.clone();
        data.payload = payload;
        input_setter.emit(KerberosInput { mode, data });
    });

    html! {
        <div class={classes!("enc-params")}>
            {build_byte_input(krb_input.data.key.clone(), set_key, None, Some("key".into()))}
            <div class={classes!("vertical")}>
                <span class={classes!("total")}>{"Key usage number"}</span>
                <input
                    type={"number"}
                    class={classes!("base-input")}
                    placeholder={"usage number"}
                    value={krb_input.data.key_usage.to_string()}
                    oninput={set_usage_number}
                />
                <span class={classes!("total")}>{get_usage_number_name(krb_input.data.key_usage)}</span>
            </div>
            {build_byte_input(krb_input.data.payload.clone(), set_payload, None, Some("payload".into()))}
            {if props.with_mode { html! {
                <div class={classes!("horizontal", "krbEncOpts")}>
                    <span class="total">{"encrypt"}</span>
                    <Switch id={"1"} setter={set_mode} state={<KrbMode as Into<bool>>::into(mode)} />
                    <span class="total">{"decrypt"}</span>
                    <span class="total">{"|"}</span>
                    <span class="total">{"raw key"}</span>
                    <Switch id={"2"} setter={set_key_source.clone()} state={*key_source} />
                    <span class="total">{"key from password"}</span>
                </div>
            }} else { html! {
                <div class={classes!("horizontal", "krbEncOpts")}>
                    <span class="total">{"raw key"}</span>
                    <Switch id={"2"} setter={set_key_source} state={*key_source} />
                    <span class="total">{"key from password"}</span>
                </div>
            }}}
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
                            <button class={classes!("jwt-util-button")} type={"submit"} onclick={generate_key_from_password}>{"Generate key"}</button>
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
    with_mode: bool,
) -> Html {
    html! {
        <KrbInput krb_input={krb_input} krb_input_setter={krb_input_setter} krb_algo={krb_algo} with_mode={with_mode} />
    }
}
