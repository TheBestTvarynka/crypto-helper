mod diff_algo;
mod diff_viewer;
mod task;

use serde::{Deserialize, Serialize};
use similar::{capture_diff_slices, Algorithm, DiffOp, TextDiff};
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::html::onchange::Event;
use yew::platform::spawn_local;
use yew::virtual_dom::VNode;
use yew_agent::oneshot::{use_oneshot_runner, OneshotProvider};
use yew::{classes, function_component, html, use_effect_with, use_state, Callback, Html, TargetCast};
use yew_hooks::{use_async, use_local_storage};

use self::diff_algo::DiffAlgo;
use self::diff_viewer::DiffViewer;
use self::task::{DiffTask, DiffTaskParams};

const DEFAULT_ORIGINAL: &str = "TheBestTvarynka
TheBestTvarynka
TheBestTvarynka";
const DEFAULT_CHANGED: &str = "thebesttravynka
thebesttravynka
thebesttravynka
";
const DEFAULT_ALGORITHM: DiffAlgo = DiffAlgo(Algorithm::Myers);

const LOCAL_STORAGE_ORIGINAL: &str = "ORIGINAL_DATA";
const LOCAL_STORAGE_ALGORITHM: &str = "ALGORITHM";
const LOCAL_STORAGE_CHANGED: &str = "CHANGED_DATA";

const ALL_ALGORITHMS: &[DiffAlgo] = &[
    DiffAlgo(Algorithm::Myers),
    DiffAlgo(Algorithm::Lcs),
    DiffAlgo(Algorithm::Patience),
];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DiffData {
    pub original: Vec<char>,
    pub changed: Vec<char>,
    pub changes: Vec<DiffOp>,
}

impl DiffData {
    pub fn empty() -> Self {
        Self {
            original: Vec::new(),
            changed: Vec::new(),
            changes: Vec::new(),
        }
    }
}

fn render_algorithm_options(current_algorithm: DiffAlgo) -> Vec<VNode> {
    ALL_ALGORITHMS
        .iter()
        .map(|algo| {
            html! {
                <option selected={current_algorithm == *algo} value={algo.to_string()}>{algo.as_ref()}</option>
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

    let original_data = original.chars().collect::<Vec<_>>();
    let changed_data = changed.chars().collect::<Vec<_>>();
    let diffs_setter = diffs.setter();
    let algo = *algorithm;
    let compute_diff = Callback::from(move |_: ()| {
        let changes = capture_diff_slices(algo.into(), &original_data, &changed_data);

        diffs_setter.set(DiffData {
            original: original_data.clone(),
            changed: changed_data.clone(),
            changes,
        });
    });

    let diff_task_params = DiffTaskParams {
        original: original.chars().collect::<Vec<_>>(),
        changed: changed.chars().collect::<Vec<_>>(),
        algo: *algorithm,
    };
    let diffs_setter = diffs.setter();
    let diff_task = use_oneshot_runner::<DiffTask>();
    let diffs_worker = {
        let diff_agent = diff_task.clone();
        Callback::from(move |_| {
            spawn_local(async move {
                let diff_data = diff_agent.run(diff_task_params).await;
                diffs_setter.set(diff_data);
            });
        })
    };

    let original_local_storage = use_local_storage::<String>(LOCAL_STORAGE_ORIGINAL.to_owned());
    let original_setter = original.setter();
    let changed_local_storage = use_local_storage::<String>(LOCAL_STORAGE_CHANGED.to_owned());
    let changed_setter = changed.setter();
    let algorithm_local_storage = use_local_storage::<String>(LOCAL_STORAGE_ALGORITHM.to_owned());
    let algorithm_setter = algorithm.setter();
    let diffs_setter = diffs.setter();
    use_effect_with([], move |_: &[(); 0]| {
        let mut flag = false;

        if let Some(original) = (*original_local_storage).as_ref() {
            original_setter.set(original.to_string());
            flag = true;
        }
        if let Some(changed) = (*changed_local_storage).as_ref() {
            changed_setter.set(changed.to_string());
            flag = true;
        }
        if let Some(raw_algorithm) = (*algorithm_local_storage).as_ref() {
            if let Ok(algorithm) = raw_algorithm.as_str().try_into() {
                algorithm_setter.set(algorithm);
                flag = true;
            }
        }

        if flag {
            diffs_setter.set(DiffData::empty());
        }
    });

    let local_storage = use_local_storage::<String>(LOCAL_STORAGE_ORIGINAL.to_owned());
    use_effect_with(original.clone(), move |original| {
        local_storage.set((*original).to_string());
    });

    let local_storage = use_local_storage::<String>(LOCAL_STORAGE_CHANGED.to_owned());
    use_effect_with(changed.clone(), move |changed| {
        local_storage.set((*changed).to_string());
    });

    let local_storage = use_local_storage::<String>(LOCAL_STORAGE_ALGORITHM.to_owned());
    use_effect_with(algorithm.clone(), move |algorithm| {
        local_storage.set((*algorithm).to_string());
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

    let onclick = {
        Callback::from(move |_| {
            debug!("started worker thread task");
            diffs_worker.emit(());
        })
    };

    let algorithm_setter = algorithm.setter();
    let on_algorithm_change = Callback::from(move |event: Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        if let Ok(algorithm) = input.value().as_str().try_into() {
            algorithm_setter.set(algorithm);
        }
    });

    let onkeydown = Callback::from(move |event: KeyboardEvent| {
        if event.ctrl_key() && event.code() == "Enter" {
            compute_diff.emit(());
        }
    });

    html! {
        <OneshotProvider<DiffTask> path="/worker.js" class={classes!("vertical", "asn1-page")} {onkeydown}>
            <div class="horizontal">
                <span>{"Diff algorithm:"}</span>
                <div>
                    <select class="base-input" onchange={on_algorithm_change}>
                        {render_algorithm_options(*algorithm)}
                    </select>
                </div>
                <span class="total">{"Alternatively, you can use this tool, which is quite good too:"}</span>
                <a class="a-link" href="https://diffviewer.vercel.app/v2">{"diffviewer.vercel.app"}</a>
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
                <span class="total">{"(ctrl+enter)"}</span>
            </div>
            <DiffViewer diff={(*diffs).clone()} />
        </OneshotProvider<DiffTask>>
    }
}
