use yew::{function_component, html, use_state, Callback, Html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct NodeOptionsProps {
    pub offset: usize,
    pub length_len: usize,
    pub data_len: usize,
}

#[function_component(NodeOptions)]
pub fn node_options(props: &NodeOptionsProps) -> Html {
    let show_options = use_state(|| false);

    let flag = *show_options;
    let show_options_setter = show_options.setter();

    let onclick = Callback::from(move |_| {
        show_options_setter.set(!flag);
    });

    html! {
        <div class="asn1-node-options-container">
            {if *show_options {html! {
                <div style="position: relative">
                    <div class="asn1-node-options">
                        <span>{format!("Offset: {}", props.offset)}</span>
                        <span>{format!("Length: {}+{}", props.length_len, props.data_len)}</span>
                    </div>
                </div>
            }} else {html! {}}}
            <button class="asn1-button-with-icon" {onclick}>
                <img src="/public/img/icons/more_vertical.png" />
            </button>
        </div>
    }
}
