macro_rules! define_string_node {
    ($name:ident) => {
        paste::paste! {
            #[derive(PartialEq, Properties, Clone)]
            pub struct [<$name NodeProps>] {
                pub node: [<Owned $name>],
                pub meta: OwnedRawAsn1EntityData,
            }

            #[allow(non_snake_case)]
            #[function_component([<$name Node >])]
            pub fn [<__fn_ $name>](props: &[<$name NodeProps>]) -> Html {
                let offset = props.meta.tag_position();
                let length_len = props.meta.length_range().len();
                let data_len = props.meta.data_range().len();

                html! {
                    <div class="terminal-asn1-node">
                        <NodeOptions node_bytes={crate::common::RcSlice::from(props.meta.raw_bytes())} {offset} {length_len} {data_len} name={stringify!($name)}/>
                        <span class="asn-simple-value">{props.node.string().to_owned()}</span>
                    </div>
                }
            }
        }
    };
}
