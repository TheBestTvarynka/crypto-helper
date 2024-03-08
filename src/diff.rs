mod diff_algo;
mod diff_viewer;

use similar::{capture_diff_slices, Algorithm, DiffOp, TextDiff};
use web_sys::HtmlInputElement;
use yew::html::onchange::Event;
use yew::virtual_dom::VNode;
use yew::{classes, function_component, html, use_state, Callback, Html, TargetCast};

use self::diff_algo::DiffAlgo;
use self::diff_viewer::DiffViewer;

const DEFAULT_ORIGINAL: &str = "TheBestTvarynka
TheBestTvarynka
TheBestTvarynka";
const DEFAULT_CHANGED: &str = "thebesttravynka
thebesttravynka
thebesttravynka
";
const DEFAULT_ALGORITHM: DiffAlgo = DiffAlgo(Algorithm::Myers);

const ALL_ALGORITHMS: &[DiffAlgo] = &[
    DiffAlgo(Algorithm::Myers),
    DiffAlgo(Algorithm::Lcs),
    DiffAlgo(Algorithm::Patience),
];

#[derive(Debug, Clone, PartialEq)]
struct DiffData {
    pub original: Vec<char>,
    pub changed: Vec<char>,
    pub changes: Vec<DiffOp>,
}

fn render_algorithm_options(current_algorithm: DiffAlgo) -> Vec<VNode> {
    ALL_ALGORITHMS
        .iter()
        .map(|algo| {
            html! {
                <option selected={current_algorithm == *algo} value={algo.to_string()}>{algo}</option>
            }
        })
        .collect()
}

#[function_component(DiffPage)]
pub fn diff_page() -> Html {
    let original = use_state(|| DEFAULT_ORIGINAL.to_owned());
    let changed = use_state(|| DEFAULT_CHANGED.to_owned());
    let algorithm = use_state(|| DEFAULT_ALGORITHM);
    let diffs = use_state(|| {
        let original = DEFAULT_ORIGINAL.chars().collect::<Vec<_>>();
        let changed = DEFAULT_CHANGED.chars().collect::<Vec<_>>();
        let changes = TextDiff::configure()
            .algorithm(DEFAULT_ALGORITHM.into())
            .newline_terminated(true)
            .diff_chars(DEFAULT_ORIGINAL, DEFAULT_CHANGED);

        DiffData {
            original,
            changed,
            changes: changes.ops().to_owned(),
        }
    });

    let original_setter = original.setter();
    let on_original_input = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        original_setter.set(input.value());
    });

    let changed_setter = changed.setter();
    let on_changed_input = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        changed_setter.set(input.value());
    });

    let original_data = original.chars().collect::<Vec<_>>();
    let changed_data = changed.chars().collect::<Vec<_>>();
    let diffs_setter = diffs.setter();
    let algo = *algorithm;
    let onclick = move |_| {
        let changes = capture_diff_slices(algo.into(), &original_data, &changed_data);

        diffs_setter.set(DiffData {
            original: original_data.clone(),
            changed: changed_data.clone(),
            changes,
        });
    };

    let algorithm_setter = algorithm.setter();
    let on_algorithm_change = Callback::from(move |event: Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        if let Ok(algorithm) = input.value().as_str().try_into() {
            algorithm_setter.set(algorithm);
        }
    });

    html! {
        <div class={classes!("vertical", "asn1-page")}>
            <div class="horizontal">
                <span>{"Diff algorithm:"}</span>
                <div>
                    <select class="base-input" onchange={on_algorithm_change}>
                        {render_algorithm_options(*algorithm)}
                    </select>
                </div>
            </div>
            <div class="horizontal">
                <textarea
                    rows="8"
                    placeholder={"original"}
                    class="base-input"
                    value={(*original).clone()}
                    oninput={on_original_input}
                />
                <textarea
                    rows="8"
                    placeholder={"changed"}
                    class="base-input"
                    value={(*changed).clone()}
                    oninput={on_changed_input}
                />
            </div>
            <div class="horizontal">
                <button class="action-button" onclick={onclick}>{"Diff"}</button>
            </div>
            <DiffViewer diff={(*diffs).clone()} />
        </div>
    }
}
