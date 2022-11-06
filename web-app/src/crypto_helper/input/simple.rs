use yew::{Html, html, classes};

use crate::crypto_helper::algorithm::Algorithm;


pub fn build_simple_input(algorithm: &Algorithm) -> Html {
    html!{
        <textarea rows="2" placeholder={format!("{:?}: hex-encoded input", algorithm)} class={classes!("base-input")} />
    }
}
