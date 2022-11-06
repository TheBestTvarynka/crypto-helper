use web_sys::HtmlInputElement;
use yew::{
    classes, function_component, html, html::onchange::Event, Callback, Html, Properties,
    TargetCast, UseStateSetter,
};

use super::algorithm::{Algorithm, SUPPORTED_ALGORITHMS};

#[derive(PartialEq, Properties)]
pub struct InfoProps {
    pub algorithm: Algorithm,
    pub set_algorithm: UseStateSetter<Algorithm>,
}

fn get_algorithm_info(algorithm: &Algorithm) -> Html {
    match algorithm {
        Algorithm::Md5(_) => html! {
            <span>
            {"Hash hex-encoded input using MD5 hashing function. "}
            <a href={"https://www.ietf.org/rfc/rfc1321.txt"}>{"RFC"}</a>{"."}
            </span>
        },
        Algorithm::Sha1(_) => html! {
            <span>{"Hash hex-encoded input using SHA1 hashing function."}
            <a href={"https://www.rfc-editor.org/rfc/rfc3174"}>{"RFC"}</a>{"."}
            </span>
        },
        Algorithm::Sha256(_) => html! {
            <span>{"Hash hex-encoded input using SHA256 hashing function."}
            <a href={"https://www.rfc-editor.org/rfc/rfc4634"}>{"RFC"}</a>{"."}
            </span>
        },
        Algorithm::Sha512(_) => html! {
            <span>{"Hash hex-encoded input using SHA512 hashing function."}
            <a href={"https://www.rfc-editor.org/rfc/rfc4634"}>{"RFC"}</a>{"."}
            </span>
        },
        Algorithm::Aes128CtsHmacSha196(_) => html! {
            <span>{"Encrypt hex-encoded data with the provided or derived key using AES128-CTS-HMAC-SHA1-96 algorithm."}
            <a href={"https://www.rfc-editor.org/rfc/rfc3961.html"}>{"RFC"}</a>{"."}
            </span>
        },
        Algorithm::Aes256CtsHmacSha196(_) => html! {
            <span>{"Encrypt hex-encoded data with the provided or derived key using AES256-CTS-HMAC-SHA1-96 algorithm."}
            <a href={"https://www.rfc-editor.org/rfc/rfc3961.html"}>{"RFC"}</a>{"."}
            </span>
        },
    }
}

#[function_component(Info)]
pub fn info(props: &InfoProps) -> Html {
    let set_algorithm = props.set_algorithm.clone();
    let onchange = Callback::from(move |event: Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(algorithm) = input.value().as_str().try_into() {
            log::info!("set new algorithm: {:?}", algorithm);
            set_algorithm.set(algorithm);
        }
    });

    html! {
        <div class={classes!("horizontal")}>
            <select onchange={onchange} class={classes!("base-input")}>{
                SUPPORTED_ALGORITHMS
                    .iter()
                    .map(|algo| html!{
                        <option selected={ &props.algorithm == *algo} value={*algo}>{algo}</option>
                    })
                    .collect::<Vec<_>>()
            }</select>
            <div class={classes!("algo-info")}>{get_algorithm_info(&props.algorithm)}</div>
        </div>
    }
}
