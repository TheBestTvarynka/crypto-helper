use web_sys::HtmlInputElement;
use yew::{Properties, Callback, Html, html, function_component, classes, TargetCast};
use yew_notifications::{use_notification, Notification, NotificationType};

use crate::crypto_helper::algorithm::{BcryptAction, BcryptHashAction, BcryptInput as BI};
use crate::common::{SwitchProps, Switch};


#[derive(PartialEq, Properties, Clone)]
pub struct BcryptInputProps {
    pub input: BI,
    pub bcrypt_input_setter: Callback<BI>,
}

#[function_component(BcryptInput)]
pub fn bcrypt_input(
    input_props: &BcryptInputProps,
) -> Html {
    let data = input_props.input.data.clone();
    let input_setter = input_props.bcrypt_input_setter.clone();
    let notifications = use_notification::<Notification>();
    let on_cost_input = Callback::from(move |event: html::oninput::Event| {
        match event.target_unchecked_into::<HtmlInputElement>().value().parse::<u32>() {
            Ok(rounds) => input_setter.emit(BI {
                action: BcryptAction::Hash(BcryptHashAction {
                    rounds,
                    salt: Vec::new(), // todo!
                }),
                data: data.clone()
            }),
            Err(err) => notifications.spawn(Notification::new(
                NotificationType::Error,
                "Bcrypt cost parsing error",
                err.to_string(),
                Notification::NOTIFICATION_LIFETIME,
            )),
        };
    });
    html! {
        <div class={classes!("formats-container")}>
            <div class={classes!("vertical")}>
            <input class={classes!("base-input")} placeholder={"password"}/>
            </div>
            <div class={classes!("vertical")}>
            <input class={classes!("base-input")} placeholder={"rounds"} oninput={on_cost_input}/>
            </div>
            <div>
            <input class={classes!("base-input")} placeholder={"salt"}/>
            </div>
        </div>
    }
}

pub fn build_bcrypt_input(
    input: BI,
    setter: Callback<BI>,
) -> Html {
    let switch = SwitchProps {
        id: "hash-verify".to_string(),
        state: false,
        setter: Callback::from(move |_b: bool| { }),
    };
    html! {
        <div>
            <BcryptInput input={input} bcrypt_input_setter={setter}/>
            <span class="total">{"hash"}</span>
            <Switch id={switch.id} state={switch.state} setter={switch.setter}/>
            <span class="total">{"verify"}</span>
        </div>
    }
}