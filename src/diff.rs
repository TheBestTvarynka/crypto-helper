mod diff_viewer;

use similar::{capture_diff_slices, Algorithm, DiffOp};
use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, use_state, Callback, Html, TargetCast};

use self::diff_viewer::DiffViewer;

const DEFAULT_ORIGINAL: &str = "TheBestTvarynka";
const DEFAULT_CHANGED: &str = "thebesttravynka";

#[derive(Debug, Clone, PartialEq)]
struct DiffData {
    pub original: Vec<char>,
    pub changed: Vec<char>,
    pub changes: Vec<DiffOp>,
}

#[function_component(DiffPage)]
pub fn diff_page() -> Html {
    let original = use_state(|| DEFAULT_ORIGINAL.to_owned());
    let changed = use_state(|| DEFAULT_CHANGED.to_owned());
    let diffs = use_state(|| {
        let original = DEFAULT_ORIGINAL.chars().collect::<Vec<_>>();
        let changed = DEFAULT_CHANGED.chars().collect::<Vec<_>>();
        let changes = capture_diff_slices(Algorithm::Myers, &original, &changed);

        DiffData {
            original,
            changed,
            changes,
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
    let onclick = move |_| {
        let changes = capture_diff_slices(Algorithm::Myers, &original_data, &changed_data);

        diffs_setter.set(DiffData {
            original: original_data.clone(),
            changed: changed_data.clone(),
            changes,
        });
    };

    html! {
        <div class={classes!("vertical", "asn1-page")}>
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
