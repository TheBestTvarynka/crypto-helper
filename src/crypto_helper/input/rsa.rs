use picky::hash::HashAlgorithm;
use yew::{classes, function_component, html, Callback, Classes, Html, Properties};

use crate::crypto_helper::algorithm::{RsaAction, RsaInput as RsaInputData, RSA_HASH_ALGOS};

#[derive(Debug, PartialEq, Properties)]
pub struct RsaInputProps {
    pub input: RsaInputData,
    pub setter: Callback<RsaInputData>,
}

fn get_action_classes(is_selected: bool) -> Classes {
    if is_selected {
        classes!("rsa-action", "selected-rsa-action")
    } else {
        classes!("rsa-action")
    }
}

fn generate_selection_action_component(
    action: &RsaAction,
    set_action: Callback<RsaAction>,
) -> Html {
    html! {
        <div class={classes!("rsa-actions-container")}>
            {RsaAction::enumerate_actions()
                .iter()
                .map(|action_name| {
                    let set_action = set_action.clone();
                    html! {
                        <span
                            class={get_action_classes(action == action_name)}
                            onclick={Callback::from(move |_| set_action.emit((*action_name).try_into().unwrap_or_default()))}
                        >
                            {action_name}
                        </span>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}

fn get_hash_selection_component(_hash_algorithm: &HashAlgorithm) -> Html {
    html! {
        <select class={classes!("base-input", "auto-width-input")}>
            {RSA_HASH_ALGOS
                .iter()
                .map(|hash_algo_name| {
                    html! {
                        <option value={hash_algo_name.to_string()}>{hash_algo_name}</option>
                    }
                })
                .collect::<Vec<_>>()}
        </select>
    }
}

fn generate_rsa_input(input: &RsaAction, set_action: Callback<RsaAction>) -> Html {
    let selected_algorithm_component = generate_selection_action_component(input, set_action);
    match input {
        RsaAction::Encrypt(_) => html! {
            <div class={classes!("vertical")}>
                {selected_algorithm_component}
                <textarea
                    rows="2"
                    placeholder={"RSA public key"}
                    class={classes!("base-input")}
                />
            </div>
        },
        RsaAction::Decrypt(_) => html! {
            <div class={classes!("vertical")}>
                {selected_algorithm_component}
                <textarea
                    rows="2"
                    placeholder={"RSA private key"}
                    class={classes!("base-input")}
                />
            </div>
        },
        RsaAction::Sign(input) => html! {
            <div class={classes!("vertical")}>
                {selected_algorithm_component}
                <div class={classes!("horizontal")}>
                    {get_hash_selection_component(&input.hash_algorithm)}
                    <textarea
                        rows="2"
                        placeholder={"RSA private key"}
                        class={classes!("base-input")}
                    />
                </div>
            </div>
        },
        RsaAction::Verify(input) => html! {
            <div class={classes!("vertical")}>
                {selected_algorithm_component}
                <div class={classes!("horizontal")}>
                    {get_hash_selection_component(&input.hash_algorithm)}
                    <textarea
                        rows="2"
                        placeholder={"RSA public key"}
                        class={classes!("base-input")}
                    />
                </div>
                <input
                    placeholder={"hex-encoded signature"}
                    class={classes!("base-input")}
                />
            </div>
        },
    }
}

#[function_component(RsaInput)]
pub fn rsa_input(props: &RsaInputProps) -> Html {
    let setter = props.setter.clone();
    let set_action = Callback::from(move |action| {
        setter.emit(RsaInputData {
            action,
            payload: "".into(),
        });
    });

    html! {
        <div class={classes!("vertical")}>
            {generate_rsa_input(&props.input.action, set_action)}
            <textarea
                rows="2"
                placeholder={"hex-encoded input"}
                class={classes!("base-input")}
            />
        </div>
    }
}

pub fn build_rsa_input(input: RsaInputData, setter: Callback<RsaInputData>) -> Html {
    html! {
        <RsaInput input={input} setter={setter} />
    }
}
