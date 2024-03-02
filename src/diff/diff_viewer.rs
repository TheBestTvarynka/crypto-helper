use yew::{function_component, html, Html, Properties};

use super::DiffData;

#[derive(PartialEq, Properties, Clone)]
pub struct DiffViewerProps {
    pub diff: DiffData,
}

#[function_component(DiffViewer)]
pub fn diff_viewer(props: &DiffViewerProps) -> Html {
    html! {
        <div>
            {format!("{:?}", props.diff.changes)}
        </div>
    }
}
