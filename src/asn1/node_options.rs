use yew::{Callback, Html, MouseEvent, Properties, function_component, html, use_state};
use yew_hooks::use_clipboard;
use yew_notifications::{Notification, NotificationType, use_notification};

use crate::common::RcSlice;

#[derive(PartialEq, Properties, Clone)]
pub struct NodeOptionsProps {
    pub name: String,
    pub node_bytes: RcSlice,
    #[prop_or_default]
    pub editor: Option<Html>,

    pub offset: usize,
    pub length_len: usize,
    pub data_len: usize,
}

#[function_component(NodeOptions)]
pub fn node_options(props: &NodeOptionsProps) -> Html {
    let show_options = use_state(|| false);
    let show_edit_panel = use_state(|| false);

    let options_flag = *show_options;
    let edit_panel_flag = *show_edit_panel;
    let show_options_setter = show_options.setter();
    let show_edit_panel_setter = show_edit_panel.setter();
    let onclick = Callback::from(move |mouse_event: MouseEvent| {
        if mouse_event.ctrl_key() {
            show_edit_panel_setter.set(!edit_panel_flag);
        } else {
            show_options_setter.set(!options_flag);
        }
    });

    let show_options_setter = show_options.setter();
    let onmouseleave_options = Callback::from(move |_| {
        show_options_setter.set(false);
    });

    let show_edit_panel_setter = show_edit_panel.setter();
    let onmouseleave_edit_panel = Callback::from(move |_| {
        show_edit_panel_setter.set(false);
    });

    let clipboard = use_clipboard();
    let notifications = use_notification::<Notification>();
    let node_bytes_len = props.node_bytes.len();
    let value_raw = props.node_bytes.with_range(props.length_len + 1, node_bytes_len);
    let copy_value = Callback::from(move |_| {
        clipboard.write_text(hex::encode(value_raw.data()));

        notifications.spawn(Notification::from_description_and_type(
            NotificationType::Info,
            "Value data copied.",
        ));
    });

    let clipboard = use_clipboard();
    let notifications = use_notification::<Notification>();
    let node_raw = props.node_bytes.clone();
    let copy_node = Callback::from(move |_| {
        clipboard.write_text(hex::encode(node_raw.data()));

        notifications.spawn(Notification::from_description_and_type(
            NotificationType::Info,
            "Node data copied.",
        ));
    });

    html! {
        <div class="asn1-node-options-container">
            {if *show_options {html! {
                <div style="position: relative">
                    <div class="asn1-node-options" onmouseleave={onmouseleave_options}>
                        <span>{format!("Offset: {}", props.offset)}</span>
                        <span>{format!("Length: {}+{}", props.length_len, props.data_len)}</span>
                        <div class="horizontal">
                            <button class="jwt-util-button" onclick={copy_value}>{"Value hex"}</button>
                            <button class="jwt-util-button" onclick={copy_node}>{"Node hex"}</button>
                        </div>
                    </div>
                </div>
            }} else {html! {}}}
            {if *show_edit_panel {html! {
                <div style="position: relative">
                    <div class="asn1-node-options" onmouseleave={onmouseleave_edit_panel} >
                        {props.editor.clone()}
                    </div>
                </div>
            }} else {html! {}}}
            <span class="asn1-node-options-name" {onclick}>{props.name.clone()}</span>
        </div>
    }
}
