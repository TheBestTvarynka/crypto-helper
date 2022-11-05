use web_sys::HtmlInputElement;
use yew::{html::onchange::Event, html, Callback, Html, Properties, function_component, UseStateSetter, TargetCast};

use super::algorithm::{Algorithm, SUPPORTED_ALGORITHMS};

#[derive(PartialEq, Properties)]
pub struct InfoProps {
    pub algorithm: Algorithm,
    pub set_algorithm: UseStateSetter<Algorithm>,
}

#[function_component(Info)]
pub fn info(props: &InfoProps) -> Html {
    let set_algorithm = props.set_algorithm.clone();
    let onclick = Callback::from(move |_| set_algorithm.set(Algorithm::Aes128CtsHmacSha196));

    let set_algorithm = props.set_algorithm.clone();

    // let oninput = Callback::from(move |event: InputEvent| {
    //     let input: HtmlInputElement = event.target_unchecked_into();
    //     let value = input.value();
    //     log::info!("{:?}", value);
    // });

    let onchange = Callback::from(move |event: Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        
        if let Ok(algorithm) = input.value().as_str().try_into() {
            log::info!("set new algorithm: {:?}", algorithm);
            set_algorithm.set(algorithm); 
        }
    });

    html! {
        <div>
            <select onchange={onchange} id={"sa"}>{
                SUPPORTED_ALGORITHMS
                    .iter()
                    .map(|algo| html!{
                        <option selected={false} value={*algo}>{algo}</option>
                    })
                    .collect::<Vec<_>>()
            }</select>
            <button onclick={onclick}>{"click me 2"}</button>
            {format!("algo: {:?}", props.algorithm)}
        </div>
    }
}
