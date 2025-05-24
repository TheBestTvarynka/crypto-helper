use std::str::FromStr;
use std::sync::OnceLock;

use asn1_parser::{ObjectIdentifier, OwnedRawAsn1EntityData};
use oid_registry::{Oid, OidRegistry};
use yew::{function_component, html, Html, Properties};

use crate::asn1::node_options::NodeOptions;
use crate::common::RcSlice;

#[derive(PartialEq, Properties, Clone)]
pub struct ObjectIdentifierProps {
    pub node: ObjectIdentifier,
    pub meta: OwnedRawAsn1EntityData,
}

#[function_component(ObjectIdentifierNode)]
pub fn bool(props: &ObjectIdentifierProps) -> Html {
    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    let formatted = props.node.format();

    html! {
        <div class="terminal-asn1-node">
            <NodeOptions node_bytes={RcSlice::from(props.meta.raw_bytes())} {offset} {length_len} {data_len} name={String::from("Object Identifier")}/>
            <span class="asn-simple-value">{&formatted}</span>
            {{
                if let Some(name) = oid_name(&formatted) {
                    let url = format!("http://www.oid-info.com/get/{formatted}");
                    html! {
                        <a class="a-link" href={url}>{name}</a>
                    }
                } else { html! {} }
            }}
        </div>
    }
}

fn get_registry() -> &'static OidRegistry<'static> {
    static REGISTRY: OnceLock<OidRegistry> = OnceLock::new();
    REGISTRY.get_or_init(|| OidRegistry::default().with_crypto())
}

fn oid_name(oid: &'_ str) -> Option<&'static str> {
    let registry = get_registry();
    Oid::from_str(oid)
        .ok()
        .and_then(|oid| registry.get(&oid))
        .map(|oid_entry| oid_entry.sn())
}
