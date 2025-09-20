mod integer;
mod number;
mod string;
mod time;

use std::fmt;

use asn1_parser::{Asn1Type, ExplicitTag, Integer, Mutable, OctetString, PrintableString, Sequence};
pub use integer::IntegerEditor;
pub use number::NumberEditor;
pub use string::StringEditor;
pub use time::GeneralizedTimeEditor;
use web_sys::HtmlInputElement;
use yew::{Callback, Html, Properties, TargetCast, UseStateSetter, function_component, html, use_state};

const OCTET_STRING: &str = "octet string";
const PRINTABLE_STRING: &str = "printable string";
const INTEGER: &str = "integer";
const SEQUENCE: &str = "sequence";
const EXPLICIT_TAG: &str = "explicit tag";

const TYPES: &[&str] = &[OCTET_STRING, PRINTABLE_STRING, INTEGER, SEQUENCE, EXPLICIT_TAG];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asn1NodeValue {
    OctetString(Vec<u8>),
    PrintableString(String),
    Integer(Vec<u8>),
    Sequence,
    ExplicitTag(u8),
}

impl TryFrom<&str> for Asn1NodeValue {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            OCTET_STRING => Self::OctetString(b"tbt".to_vec()),
            PRINTABLE_STRING => Self::PrintableString(String::from("tbt")),
            INTEGER => Self::Integer(vec![5]),
            SEQUENCE => Self::Sequence,
            EXPLICIT_TAG => Self::ExplicitTag(1),
            _ => return Err(()),
        })
    }
}

impl From<&Asn1NodeValue> for &str {
    fn from(value: &Asn1NodeValue) -> Self {
        match value {
            Asn1NodeValue::OctetString(_) => OCTET_STRING,
            Asn1NodeValue::PrintableString(_) => PRINTABLE_STRING,
            Asn1NodeValue::Integer(_) => INTEGER,
            Asn1NodeValue::Sequence => SEQUENCE,
            Asn1NodeValue::ExplicitTag(_) => EXPLICIT_TAG,
        }
    }
}

impl fmt::Display for Asn1NodeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name: &str = self.into();
        f.write_str(name)
    }
}

impl From<Asn1NodeValue> for Asn1Type {
    fn from(value: Asn1NodeValue) -> Self {
        match value {
            Asn1NodeValue::OctetString(data) => Asn1Type::OctetString(Mutable::new(OctetString::new(data))),
            Asn1NodeValue::PrintableString(data) => Asn1Type::PrintableString(Mutable::new(PrintableString::new(data))),
            Asn1NodeValue::Integer(data) => Asn1Type::Integer(Mutable::new(Integer::from(data))),
            Asn1NodeValue::Sequence => Asn1Type::Sequence(Mutable::new(Sequence::new(Vec::new()))),
            Asn1NodeValue::ExplicitTag(tag) => Asn1Type::ExplicitTag(Mutable::new(ExplicitTag::new(tag, Vec::new()))),
        }
    }
}

fn editor(asn1_node: Asn1NodeValue, asn1_node_setter: UseStateSetter<Asn1NodeValue>) -> Html {
    match asn1_node {
        Asn1NodeValue::OctetString(_data) => {
            // TODO
            html! {}
        }
        Asn1NodeValue::PrintableString(value) => html! {
            <StringEditor {value} setter={Callback::from(move |data| asn1_node_setter.set(Asn1NodeValue::PrintableString(data)))} />
        },
        Asn1NodeValue::Integer(value) => html! {
            <IntegerEditor {value} setter={Callback::from(move |data| asn1_node_setter.set(Asn1NodeValue::Integer(data)))} />
        },
        Asn1NodeValue::Sequence => html! {
            <span>{"No editor available for SEQUENCE"}</span>
        },
        Asn1NodeValue::ExplicitTag(tag) => html! {
            <NumberEditor
                value={isize::from(tag)}
                setter={Callback::from(move |number| asn1_node_setter.set(Asn1NodeValue::ExplicitTag(number as u8)))}
                min={1}
                max={30}
            />
        },
    }
}

#[derive(PartialEq, Properties, Clone, Debug)]
pub struct Asn1NodeValueOptions {
    pub add_node: Callback<Asn1Type>,
    pub cancel: Callback<()>,
}

#[function_component(Asn1NodeValueEditor)]
pub fn asn1_node_value_editor(props: &Asn1NodeValueOptions) -> Html {
    let asn1_node = use_state(|| Asn1NodeValue::PrintableString(String::from("tbt")));

    let asn1_node_setter = asn1_node.setter();
    let oninput = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let value = input.value().to_ascii_lowercase();

        for node_type in TYPES {
            if node_type.starts_with(&value) {
                asn1_node_setter.set(Asn1NodeValue::try_from(*node_type).expect("valid ASN1 type name"));
            }
        }
    });

    let asn1_node_value = (*asn1_node).clone();
    let add_node = props.add_node.clone();
    let add_node_click = move |_| {
        add_node.emit(Asn1Type::from(asn1_node_value.clone()));
    };

    let cancel = props.cancel.clone();
    let cancel_click = move |_| {
        cancel.emit(());
    };

    html! {
        <div class="vertical">
            <textarea
                class={"modal-input"}
                cols={"30"}
                placeholder={"e.g. octet string"}
                {oninput}
                rows="1"
            />
            <div>
                <span class="asn-simple-value">{(*asn1_node).to_string()}</span>
            </div>
            {editor((*asn1_node).clone(), asn1_node.setter())}
            <div class="horizontal">
                <button class="jwt-util-button" onclick={add_node_click}>{"Add node"}</button>
                <button class="asn1-button-with-icon" onclick={cancel_click} title="Cancel">
                    <img src="/public/img/icons/close.png" />
                </button>
            </div>
        </div>
    }
}
