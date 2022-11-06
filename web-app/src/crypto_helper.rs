use yew::{classes, function_component, html, use_state, Html};

mod algorithm;
mod info;
mod input;
mod output;

use info::Info;
use input::Input;
use output::Output;

use self::algorithm::Algorithm;

#[function_component(CryptoHelper)]
pub fn crypto_helper() -> Html {
    //
    let algorithm = use_state(Algorithm::default);
    let output = use_state(Vec::new);

    html! {
        <article class={classes!("vertical")}>
            <Info set_algorithm={algorithm.setter()} algorithm={(*algorithm).clone()} />
            <Input algorithm={(*algorithm).clone()} setter={algorithm.setter()} />
            <Output algorithm={(*algorithm).clone()} output={(*output).clone()} />
        </article>
    }
}
