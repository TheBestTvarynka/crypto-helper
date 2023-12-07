use asn1_parser::{OwnedApplicationTag, OwnedExplicitTag};
use yew::{function_component, html, Callback, Html, Properties};

use crate::asn1::scheme::build_asn1_schema;
use crate::asn1::HighlightAction;

#[derive(PartialEq, Properties, Clone)]
pub struct ExplicitTagProps {
    pub node: OwnedExplicitTag,
    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
}

#[function_component(ExplicitTagNode)]
pub fn explicit_tag(props: &ExplicitTagProps) -> Html {
    html! {
        <div>
            <div class="asn1-constructor-header">
                <span>{format!("[{}]", props.node.tag_number())}</span>
            </div>
            <div class="asn1-constructor-body">
                {build_asn1_schema(props.node.inner(), &props.cur_node, &props.set_cur_node)}
            </div>
        </div>
    }
}
#[derive(PartialEq, Properties, Clone)]
pub struct ApplicationTagProps {
    pub node: OwnedApplicationTag,
    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
}

#[function_component(ApplicationTagNode)]
pub fn application_tag(props: &ApplicationTagProps) -> Html {
    html! {
        <div>
            <div class="asn1-constructor-header">
                <span>{format!("Application {}", props.node.tag_number())}</span>
            </div>
            <div class="asn1-constructor-body">
                {build_asn1_schema(props.node.inner(), &props.cur_node, &props.set_cur_node)}
            </div>
        </div>
    }
}
