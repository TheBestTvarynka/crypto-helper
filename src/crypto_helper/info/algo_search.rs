use web_sys::{Event, FocusEvent, HtmlInputElement, InputEvent, KeyboardEvent, MouseEvent};
use yew::{
    classes, function_component, html, use_effect_with_deps, use_state, Callback, Html, Properties,
    TargetCast, UseStateSetter,
};

use crate::crypto_helper::algorithm::{Algorithm, SUPPORTED_ALGORITHMS};

fn search_algorithms<'a>(pattern: String) -> Vec<&'a str> {
    if pattern.is_empty() {
        return Vec::new();
    }

    let mut algorithms = Vec::new();

    let pattern = pattern.to_uppercase();
    for algorithm in SUPPORTED_ALGORITHMS.iter() {
        if algorithm.contains(&pattern) {
            algorithms.push(*algorithm);
        }
    }

    algorithms
}

fn get_onclick_hangle(
    algorithm: &str,
    set_algorithm: UseStateSetter<Algorithm>,
    set_algos: UseStateSetter<Vec<&'static str>>,
) -> Callback<MouseEvent> {
    let algorithm = algorithm.to_owned();
    Callback::from(move |_| {
        set_algorithm.set(algorithm.as_str().try_into().unwrap_or_default());
        set_algos.set(Vec::new());
    })
}

#[derive(Properties, PartialEq)]
pub struct AlgoSearchProps {
    pub set_algorithm: UseStateSetter<Algorithm>,
}

#[function_component(AlgoSearch)]
pub fn algo_search(props: &AlgoSearchProps) -> Html {
    let pattern = use_state(String::new);
    let algos = use_state(Vec::new);

    let algos_setter = algos.setter();
    use_effect_with_deps(
        move |pattern| {
            let pattern = (**pattern).clone();
            algos_setter.set(search_algorithms(pattern));
        },
        pattern.clone(),
    );

    let pattern_setter = pattern.setter();
    let oninput = Callback::from(move |event: InputEvent| {
        let input: HtmlInputElement = event.target_unchecked_into();
        pattern_setter.set(input.value());
    });

    let pattern_setter = pattern.setter();
    let onchange = Callback::from(move |event: Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        pattern_setter.set(input.value());
    });

    let pattern_value = (*pattern).clone();
    let algos_setter = algos.setter();
    let onfocus = Callback::from(move |_event: FocusEvent| {
        algos_setter.set(search_algorithms(pattern_value.clone()));
    });

    let algos_setter = algos.setter();
    let on_background_click = Callback::from(move |_| {
        algos_setter.set(Vec::new());
    });

    let algos_value = (*algos).clone();
    let algo_setter = props.set_algorithm.clone();
    let algos_setter = algos.setter();
    let onkeydown = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "Enter" {
            if let Some(algo) = algos_value.first() {
                algo_setter.set((*algo).try_into().unwrap_or_default());
                algos_setter.set(Vec::new());
            }
        }
    });

    html! {
        <div class={classes!("vertical")}>
            <input
                placeholder={"algo name"}
                class={classes!("base-input")}
                onfocus={onfocus}
                value={(*pattern).clone()}
                {oninput}
                {onchange}
                {onkeydown}
            />
            <div class={classes!("search-result-container")}>
                <div class={classes!("vertical", "search-result")}>
                {
                    if !(*algos).is_empty() {
                        html! { <div class={classes!("algo-search-background")} onclick={on_background_click} /> }
                    } else {
                        html! {}
                    }
                }
                {
                    (*algos)
                        .iter()
                        .map(|algo| html!{
                            <span
                                class={classes!("sr")}
                                onclick={get_onclick_hangle(algo, props.set_algorithm.clone(), algos.setter())}
                            >
                                {algo}
                            </span>
                        })
                        .collect::<Vec<_>>()
                }
                </div>
            </div>
        </div>
    }
}
