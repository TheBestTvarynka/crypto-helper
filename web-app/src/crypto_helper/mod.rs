use yew::{html, Html, function_component, use_state};

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
    let algorithm = use_state(|| Algorithm::Sha1);

    html!{
        <div>
            <Info set_algorithm={algorithm.setter()} algorithm={(*algorithm).clone()} />
            <Input algorithm={(*algorithm).clone()} />
            <Output algorithm={(*algorithm).clone()} />
        </div>
    }
}
