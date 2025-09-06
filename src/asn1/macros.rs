macro_rules! define_string_node {
    ($name:ident) => {
        paste::paste! {
            #[derive(PartialEq, Properties, Clone)]
            pub struct [<$name NodeProps>] {
                pub node: Mutable<$name>,
                pub meta: RawAsn1EntityData,
                pub re_encode: Callback<()>,
            }

            #[allow(non_snake_case)]
            #[function_component([<$name Node >])]
            pub fn [<__fn_ $name>](props: &[<$name NodeProps>]) -> Html {
                use crate::asn1::editor::StringEditor;

                let offset = props.meta.tag_position();
                let length_len = props.meta.length_range().len();
                let data_len = props.meta.data_range().len();

                let node = props.node.clone();
                let re_encode = props.re_encode.clone();
                let setter = Callback::from(move |value: String| {
                    node.get_mut().set_string(value.clone());
                    re_encode.emit(());
                });

                html! {
                    <div class="terminal-asn1-node">
                        <NodeOptions
                            node_bytes={crate::common::RcSlice::from(props.meta.raw_bytes())}
                            {offset}
                            {length_len}
                            {data_len}
                            name={stringify!($name)}
                            editor={Some(html! {
                                <StringEditor
                                    value={props.node.get().string().to_owned()}
                                    {setter}
                                />
                            })}
                        />
                        <span class="asn-simple-value">{props.node.get().string().to_owned()}</span>
                    </div>
                }
            }
        }
    };
}
