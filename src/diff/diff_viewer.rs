use similar::DiffOp;
use yew::virtual_dom::VNode;
use yew::{function_component, html, Html, Properties};

use super::DiffData;

#[derive(PartialEq, Properties, Clone)]
pub struct DiffViewerProps {
    pub diff: DiffData,
}

fn text_spans(chars: &[char], class: &'static str, line_number: &mut usize) -> Vec<VNode> {
    chars
        .iter()
        .flat_map(|c| {
            if *c == '\n' {
                vec![
                    html! {
                        <span class="diff-new-line" />
                    },
                    html! {
                        <span class="diff-line-number">{{
                            *line_number += 1;
                            *line_number - 1
                        }}</span>
                    },
                ]
            } else {
                vec![html! {
                    <span class={class}>{c}</span>
                }]
            }
        })
        .collect()
}

fn render_changes(diff: &DiffData) -> Vec<VNode> {
    let mut line_number = 2;

    let old = diff.original.as_slice();
    let new = diff.changed.as_slice();

    let mut changes = diff
        .changes
        .iter()
        .flat_map(|op| match op {
            DiffOp::Equal {
                old_index,
                len,
                new_index: _,
            } => text_spans(&old[*old_index..*old_index + *len], "diff-unchanged", &mut line_number),
            DiffOp::Delete {
                old_index,
                old_len,
                new_index: _,
            } => text_spans(&old[*old_index..*old_index + *old_len], "diff-remove", &mut line_number),
            DiffOp::Insert {
                old_index: _,
                new_index,
                new_len,
            } => text_spans(&new[*new_index..*new_index + *new_len], "diff-insert", &mut line_number),
            DiffOp::Replace {
                old_index,
                old_len,
                new_index,
                new_len,
            } => {
                let mut spans = text_spans(&old[*old_index..*old_index + *old_len], "diff-remove", &mut line_number);
                spans.extend_from_slice(&text_spans(
                    &new[*new_index..*new_index + *new_len],
                    "diff-insert",
                    &mut line_number,
                ));
                spans
            }
        })
        .collect::<Vec<_>>();
    changes.insert(
        0,
        html! {
            <span class="diff-line-number">{"1"}</span>
        },
    );
    let _ = changes.remove(changes.len() - 1);
    changes
}

#[function_component(DiffViewer)]
pub fn diff_viewer(props: &DiffViewerProps) -> Html {
    html! {
        <div class="diff-changes-container">
            {render_changes(&props.diff)}
        </div>
    }
}
