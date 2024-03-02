use similar::DiffOp;
use yew::virtual_dom::VNode;
use yew::{function_component, html, Html, Properties};

use super::DiffData;

#[derive(PartialEq, Properties, Clone)]
pub struct DiffViewerProps {
    pub diff: DiffData,
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
            } => vec![html! {
                <span>{old[*old_index..*old_index + *len].iter().collect::<String>()}</span>
            }],
            DiffOp::Delete {
                old_index,
                old_len,
                new_index: _,
            } => vec![html! {
                <span class="diff-remove">{old[*old_index..*old_index + *old_len].iter().collect::<String>()}</span>
            }],
            DiffOp::Insert {
                old_index: _,
                new_index,
                new_len,
            } => vec![html! {
                <span class="diff-insert">{new[*new_index..*new_index + *new_len].iter().collect::<String>()}</span>
            }],
            DiffOp::Replace {
                old_index,
                old_len,
                new_index,
                new_len,
            } => vec![
                html! {
                    <span class="diff-remove">{old[*old_index..*old_index + *old_len].iter().collect::<String>()}</span>
                },
                html! {
                    <span class="diff-insert">{new[*new_index..*new_index + *new_len].iter().collect::<String>()}</span>
                },
            ],
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
