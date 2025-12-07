mod integer;
mod null;
mod number;
mod string;
mod time;

use std::fmt;

use ::time::OffsetDateTime;
use asn1_parser::{
    Asn1, Asn1Decoder, Asn1Type, BitString, BmpString, Bool, Day, ExplicitTag, GeneralString, GeneralizedTime,
    GtSecond, GtYear, Hour, IA5String, Integer, Minute, Month, Mutable, NumericString, ObjectIdentifier, OctetString,
    PrintableString, Second, Sequence, Set, UtcTime, Utf8String, VisibleString, Year,
};
use web_sys::HtmlInputElement;
use yew::{Callback, Html, Properties, TargetCast, UseStateSetter, function_component, html, use_state};

pub use self::integer::{BYTES_FORMATS, INTEGER_FORMATS, IntegerEditor};
pub use self::null::NullEditor;
pub use self::number::NumberEditor;
pub use self::string::StringEditor;
pub use self::time::{GeneralizedTimeEditor, UtcTimeEditor};
use crate::asn1::scheme::validate_oid;
use crate::common::Switch;

const OCTET_STRING: &str = "octet string";
const PRINTABLE_STRING: &str = "printable string";
const INTEGER: &str = "integer";
const SEQUENCE: &str = "sequence";
const SET: &str = "set";
const EXPLICIT_TAG: &str = "explicit tag";
const GENERALIZED_TIME: &str = "generalized time";
const UTC_TIME: &str = "utc time";
const UTF8_STRING: &str = "utf8 string";
const GENERAL_STRING: &str = "general string";
const IA5_STRING: &str = "ia5 string";
const VISIBLE_STRING: &str = "visible string";
const NUMERIC_STRING: &str = "numeric string";
const BMP_STRING: &str = "bmp string";
const BIT_STRING: &str = "bit string";
const OBJECT_IDENTIFIER: &str = "object identifier";
const BOOL: &str = "bool";
const RAW: &str = "raw";

const TYPES: &[&str] = &[
    OCTET_STRING,
    PRINTABLE_STRING,
    UTF8_STRING,
    GENERAL_STRING,
    IA5_STRING,
    VISIBLE_STRING,
    NUMERIC_STRING,
    INTEGER,
    SEQUENCE,
    SET,
    EXPLICIT_TAG,
    GENERALIZED_TIME,
    UTC_TIME,
    BMP_STRING,
    BIT_STRING,
    OBJECT_IDENTIFIER,
    BOOL,
    RAW,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asn1NodeValue {
    Raw(Vec<u8>),
    OctetString(Vec<u8>),
    BitString(Vec<u8>),
    PrintableString(String),
    Utf8String(String),
    GeneralString(String),
    Ia5String(String),
    VisibleString(String),
    NumericString(String),
    Integer(Vec<u8>),
    Sequence,
    Set,
    ExplicitTag(u8),
    GeneralizedTime(GeneralizedTime),
    UtcTime(UtcTime),
    BmpString(String),
    ObjectIdentifier(String),
    Bool(bool),
}

impl TryFrom<&str> for Asn1NodeValue {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            RAW => Self::Raw(
                // Sequence { Utf8String: "TheBestTvarynka" }
                vec![
                    48, 17, 12, 15, 84, 104, 101, 66, 101, 115, 116, 84, 118, 97, 114, 121, 110, 107, 97,
                ],
            ),
            OCTET_STRING => Self::OctetString(b"tbt".to_vec()),
            BIT_STRING => Self::BitString(b"tbt".to_vec()),
            PRINTABLE_STRING => Self::PrintableString(String::from("tbt")),
            INTEGER => Self::Integer(vec![5]),
            SEQUENCE => Self::Sequence,
            SET => Self::Set,
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
            UTF8_STRING => Self::Utf8String(String::from("tbt")),
            GENERAL_STRING => Self::GeneralString(String::from("tbt")),
            IA5_STRING => Self::Ia5String(String::from("tbt")),
            VISIBLE_STRING => Self::VisibleString(String::from("tbt")),
            NUMERIC_STRING => Self::NumericString(String::from("12345")),
            BMP_STRING => Self::BmpString(String::from("tbt")),
            OBJECT_IDENTIFIER => Self::ObjectIdentifier(String::from("2.5.4.6")),
            BOOL => Self::Bool(false),
            _ => return Err(()),
        })
    }
}

impl From<&Asn1NodeValue> for &str {
    fn from(value: &Asn1NodeValue) -> Self {
        match value {
            Asn1NodeValue::Raw(_) => RAW,
            Asn1NodeValue::OctetString(_) => OCTET_STRING,
            Asn1NodeValue::BitString(_) => BIT_STRING,
            Asn1NodeValue::PrintableString(_) => PRINTABLE_STRING,
            Asn1NodeValue::Integer(_) => INTEGER,
            Asn1NodeValue::Sequence => SEQUENCE,
            Asn1NodeValue::Set => SET,
            Asn1NodeValue::ExplicitTag(_) => EXPLICIT_TAG,
            Asn1NodeValue::GeneralizedTime(_) => GENERALIZED_TIME,
            Asn1NodeValue::UtcTime(_) => UTC_TIME,
            Asn1NodeValue::Utf8String(_) => UTF8_STRING,
            Asn1NodeValue::GeneralString(_) => GENERAL_STRING,
            Asn1NodeValue::Ia5String(_) => IA5_STRING,
            Asn1NodeValue::VisibleString(_) => VISIBLE_STRING,
            Asn1NodeValue::NumericString(_) => NUMERIC_STRING,
            Asn1NodeValue::BmpString(_) => BMP_STRING,
            Asn1NodeValue::ObjectIdentifier(_) => OBJECT_IDENTIFIER,
            Asn1NodeValue::Bool(_) => BOOL,
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
            Asn1NodeValue::Raw(data) => Asn1::decode_buff(&data)
                .map(Asn1::into_inner_asn1)
                .unwrap_or_else(|err| {
                    error!(?err, "Failed to parse provided asn1 buffer");

                    Asn1Type::Utf8String(Mutable::new(Utf8String::new(String::from(
                        "Failed to parse provided asn1 buffer",
                    ))))
                }),
            Asn1NodeValue::OctetString(data) => Asn1Type::OctetString(Mutable::new(OctetString::new(data))),
            Asn1NodeValue::BitString(data) => Asn1Type::BitString(Mutable::new(BitString::from(data))),
            Asn1NodeValue::PrintableString(data) => Asn1Type::PrintableString(Mutable::new(PrintableString::new(data))),
            Asn1NodeValue::Integer(data) => Asn1Type::Integer(Mutable::new(Integer::from(data))),
            Asn1NodeValue::Sequence => Asn1Type::Sequence(Mutable::new(Sequence::new(Vec::new()))),
            Asn1NodeValue::Set => Asn1Type::Set(Mutable::new(Set::new(Vec::new()))),
            Asn1NodeValue::ExplicitTag(tag) => Asn1Type::ExplicitTag(Mutable::new(ExplicitTag::new(tag, Vec::new()))),
            Asn1NodeValue::GeneralizedTime(data) => Asn1Type::GeneralizedTime(Mutable::new(data)),
            Asn1NodeValue::UtcTime(data) => Asn1Type::UtcTime(Mutable::new(data)),
            Asn1NodeValue::Utf8String(data) => Asn1Type::Utf8String(Mutable::new(Utf8String::new(data))),
            Asn1NodeValue::GeneralString(data) => Asn1Type::GeneralString(Mutable::new(GeneralString::new(data))),
            Asn1NodeValue::Ia5String(data) => Asn1Type::IA5String(Mutable::new(IA5String::new(data))),
            Asn1NodeValue::VisibleString(data) => Asn1Type::VisibleString(Mutable::new(VisibleString::new(data))),
            Asn1NodeValue::NumericString(data) => Asn1Type::NumericString(Mutable::new(NumericString::new(data))),
            Asn1NodeValue::BmpString(data) => Asn1Type::BmpString(Mutable::new(BmpString::new(
                data.encode_utf16().flat_map(|c| c.to_be_bytes()).collect(),
            ))),
            Asn1NodeValue::ObjectIdentifier(data) => {
                Asn1Type::ObjectIdentifier(Mutable::new(ObjectIdentifier::new_unchecked(&data)))
            }
            Asn1NodeValue::Bool(data) => Asn1Type::Bool(Mutable::new(Bool::from(data))),
        }
    }
}

fn editor(asn1_node: Asn1NodeValue, asn1_node_setter: UseStateSetter<Asn1NodeValue>) -> Html {
    match asn1_node {
        Asn1NodeValue::Raw(value) => html! {
            <IntegerEditor
                {value}
                setter={Callback::from(move |data| asn1_node_setter.set(Asn1NodeValue::Raw(data)))}
                formats={BYTES_FORMATS}
            />
        },
        Asn1NodeValue::Bool(value) => html! {
            <Switch id={"bool_node_creation"} state={value} setter={Callback::from(move |data| asn1_node_setter.set(Asn1NodeValue::Bool(data)))} />
        },
        Asn1NodeValue::OctetString(value) => html! {
            <IntegerEditor
                {value}
                setter={Callback::from(move |data| asn1_node_setter.set(Asn1NodeValue::OctetString(data)))}
                formats={BYTES_FORMATS}
            />
        },
        Asn1NodeValue::BitString(value) => html! {
            <IntegerEditor
                {value}
                setter={Callback::from(move |data| asn1_node_setter.set(Asn1NodeValue::BitString(data)))}
                formats={BYTES_FORMATS}
            />
        },
        Asn1NodeValue::ObjectIdentifier(value) => html! {
            <StringEditor
                {value}
                setter={Callback::from(move |data| asn1_node_setter.set(Asn1NodeValue::ObjectIdentifier(data)))}
                validator={Callback::from(move |s: String| validate_oid(&s))}
            />
        },
        Asn1NodeValue::BmpString(value) => html! {
            <StringEditor
                {value}
                setter={Callback::from(move |data| asn1_node_setter.set(Asn1NodeValue::PrintableString(data)))}
                validator={Callback::from(move |_| true)}
            />
        },
        Asn1NodeValue::PrintableString(value) => html! {
            <StringEditor
                {value}
                setter={Callback::from(move |data| asn1_node_setter.set(Asn1NodeValue::PrintableString(data)))}
                validator={Callback::from(move |s: String| asn1_parser::validate_printable(&s))}
            />
        },
        Asn1NodeValue::Utf8String(value) => html! {
            <StringEditor
                {value}
                setter={Callback::from(move |data| asn1_node_setter.set(Asn1NodeValue::Utf8String(data)))}
                validator={Callback::from(move |s: String| asn1_parser::validate_utf8(&s))}
            />
        },
        Asn1NodeValue::GeneralString(value) => html! {
            <StringEditor
                {value}
                setter={Callback::from(move |data| asn1_node_setter.set(Asn1NodeValue::GeneralString(data)))}
                validator={Callback::from(move |s: String| asn1_parser::validate_general(&s))}
            />
        },
        Asn1NodeValue::Ia5String(value) => html! {
            <StringEditor
                {value}
                setter={Callback::from(move |data| asn1_node_setter.set(Asn1NodeValue::Ia5String(data)))}
                validator={Callback::from(move |s: String| asn1_parser::validate_ia5(&s))}
            />
        },
        Asn1NodeValue::VisibleString(value) => html! {
            <StringEditor
                {value}
                setter={Callback::from(move |data| asn1_node_setter.set(Asn1NodeValue::VisibleString(data)))}
                validator={Callback::from(move |s: String| asn1_parser::validate_visible(&s))}
            />
        },
        Asn1NodeValue::NumericString(value) => html! {
            <StringEditor
                {value}
                setter={Callback::from(move |data| asn1_node_setter.set(Asn1NodeValue::NumericString(data)))}
                validator={Callback::from(move |s: String| asn1_parser::validate_numeric(&s))}
            />
        },
        Asn1NodeValue::Integer(value) => html! {
            <IntegerEditor
                {value}
                setter={Callback::from(move |data| asn1_node_setter.set(Asn1NodeValue::Integer(data)))}
                formats={INTEGER_FORMATS}
            />
        },
        Asn1NodeValue::Sequence => html! {
            <span />
        },
        Asn1NodeValue::Set => html! {
            <span />
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
pub struct NodeValueOptions {
    pub add_node: Callback<Asn1Type>,
    pub cancel: Callback<()>,
}

#[function_component(NodeValueEditor)]
pub fn node_value_editor(props: &NodeValueOptions) -> Html {
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

#[derive(PartialEq, Properties, Clone, Debug)]
pub struct RemoveNodeConfirmationProps {
    pub remove_node: Callback<()>,
}

#[function_component(RemoveNodeConfirmation)]
pub fn remove_node_confirmation(props: &RemoveNodeConfirmationProps) -> Html {
    let remove_node = props.remove_node.clone();
    let onclick = move |_| remove_node.emit(());

    html! {
        <div class="horizontal">
            <button class="jwt-util-button" {onclick}>{"Yes, I am sure"}</button>
        </div>
    }
}
