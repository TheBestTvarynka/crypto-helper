mod integer;
mod number;
mod string;
mod time;

use std::fmt;

use ::time::OffsetDateTime;
use asn1_parser::{
    Asn1Type, Day, ExplicitTag, GeneralizedTime, GtSecond, GtYear, Hour, Integer, Minute, Month, Mutable, OctetString,
    PrintableString, Second, Sequence, UtcTime, Year,
};
use web_sys::HtmlInputElement;
use yew::{Callback, Html, Properties, TargetCast, UseStateSetter, function_component, html, use_state};

pub use self::integer::IntegerEditor;
pub use self::number::NumberEditor;
pub use self::string::StringEditor;
pub use self::time::{GeneralizedTimeEditor, UtcTimeEditor};

const OCTET_STRING: &str = "octet string";
const PRINTABLE_STRING: &str = "printable string";
const INTEGER: &str = "integer";
const SEQUENCE: &str = "sequence";
const EXPLICIT_TAG: &str = "explicit tag";
const GENERALIZED_TIME: &str = "generalized time";
const UTC_TIME: &str = "utc time";

const TYPES: &[&str] = &[
    OCTET_STRING,
    PRINTABLE_STRING,
    INTEGER,
    SEQUENCE,
    EXPLICIT_TAG,
    GENERALIZED_TIME,
    UTC_TIME,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asn1NodeValue {
    OctetString(Vec<u8>),
    PrintableString(String),
    Integer(Vec<u8>),
    Sequence,
    ExplicitTag(u8),
    GeneralizedTime(GeneralizedTime),
    UtcTime(UtcTime),
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
            GENERALIZED_TIME => Self::GeneralizedTime(GeneralizedTime::new(
                GtYear::new(2025),
                Month::try_from(9).expect("valid month"),
                Day::try_from(20).expect("valid day"),
                Hour::try_from(18).expect("valid hour"),
                Minute::try_from(35).expect("valid minute"),
                GtSecond::try_from(3.56).expect("valid second"),
                None,
            )),
            UTC_TIME => {
                let now = OffsetDateTime::now_utc();
                Self::UtcTime(UtcTime {
                    year: Year::try_from((now.year() % 100) as u8).expect("valid year"),
                    month: Month::try_from(now.month() as u8).expect("valid month"),
                    day: Day::try_from(now.day()).expect("valid day"),
                    hour: Hour::try_from(now.hour()).expect("valid hour"),
                    minute: Minute::try_from(now.minute()).expect("valid minute"),
                    second: Some(Second::try_from(now.second()).expect("valid second")),
                })
            }
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
            Asn1NodeValue::GeneralizedTime(_) => GENERALIZED_TIME,
            Asn1NodeValue::UtcTime(_) => UTC_TIME,
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
            Asn1NodeValue::GeneralizedTime(data) => Asn1Type::GeneralizedTime(Mutable::new(data)),
            Asn1NodeValue::UtcTime(data) => Asn1Type::UtcTime(Mutable::new(data)),
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
        Asn1NodeValue::GeneralizedTime(value) => html! {
            <GeneralizedTimeEditor
                value={value}
                setter={Callback::from(move |data| asn1_node_setter.set(Asn1NodeValue::GeneralizedTime(data)))}
            />
        },
        Asn1NodeValue::UtcTime(value) => html! {
            <UtcTimeEditor
                value={value}
                setter={Callback::from(move |data| asn1_node_setter.set(Asn1NodeValue::UtcTime(data)))}
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
