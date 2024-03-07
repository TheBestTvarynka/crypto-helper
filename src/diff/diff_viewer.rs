use similar::DiffOp;
use yew::virtual_dom::VNode;
use yew::{function_component, html, Html, Properties};

use super::DiffData;

#[derive(PartialEq, Properties, Clone)]
pub struct DiffViewerProps {
    pub diff: DiffData,
}

fn text_spans(chars: &[char], class: &'static str) -> Vec<VNode> {
    chars
        .iter()
        .map(|c| {
            if *c == '\n' {
                html! {
                    <span class="diff-new-line" />
                }
            } else {
                html! {
                    <span class={class}>{c}</span>
                }
            }
        })
        .collect()
}

fn render_changes(diff: &DiffData) -> Vec<VNode> {
    let old = diff.original.as_slice();
    let new = diff.changed.as_slice();
    diff.changes
        .iter()
        .flat_map(|op| match op {
            DiffOp::Equal {
                old_index,
                len,
                new_index: _,
            } => text_spans(&old[*old_index..*old_index + *len], ""),
            DiffOp::Delete {
                old_index,
                old_len,
                new_index: _,
            } => text_spans(&old[*old_index..*old_index + *old_len], "diff-remove"),
            DiffOp::Insert {
                old_index: _,
                new_index,
                new_len,
            } => text_spans(&new[*new_index..*new_index + *new_len], "diff-insert"),
            DiffOp::Replace {
                old_index,
                old_len,
                new_index,
                new_len,
            } => {
                let mut spans = text_spans(&old[*old_index..*old_index + *old_len], "diff-remove");
                spans.extend_from_slice(&text_spans(&new[*new_index..*new_index + *new_len], "diff-insert"));
                spans
            }
        })
        .collect()
}

#[function_component(DiffViewer)]
pub fn diff_viewer(props: &DiffViewerProps) -> Html {
    html! {
        <div class="diff-changes-container">
            {render_changes(&props.diff)}
        </div>
    }
}
