use asn1_parser::{GeneralizedTime, Mutable, RawAsn1EntityData, UtcTime};
use yew::{Html, Properties, function_component, html};

use crate::asn1::node_options::NodeOptions;
use crate::common::RcSlice;

#[derive(PartialEq, Properties, Clone)]
pub struct UtcTimeNodeProps {
    pub node: Mutable<UtcTime>,
    pub meta: RawAsn1EntityData,
}

#[function_component(UtcTimeNode)]
pub fn utc_time_string(props: &UtcTimeNodeProps) -> Html {
    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    html! {
        <div class="terminal-asn1-node">
            <NodeOptions node_bytes={RcSlice::from(props.meta.raw_bytes())} {offset} {length_len} {data_len} name={String::from("UtcTime")}/>
            <span class="asn-simple-value">{format_utc_time(&props.node.get())}</span>
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct GeneralizedTimeNodeProps {
    pub node: Mutable<GeneralizedTime>,
    pub meta: RawAsn1EntityData,
}

#[function_component(GeneralizedTimeNode)]
pub fn general_time_string(props: &GeneralizedTimeNodeProps) -> Html {
    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    html! {
        <div class="terminal-asn1-node">
            <NodeOptions node_bytes={RcSlice::from(props.meta.raw_bytes())} {offset} {length_len} {data_len} name={String::from("GeneralizedTime")}/>
            <span class="asn-simple-value">{format_generalized_time(&props.node.get())}</span>
        </div>
    }
}

fn format_utc_time(utc_time: &UtcTime) -> String {
    use time::OffsetDateTime;

    let mut formatted = String::new();

    let current_year = (OffsetDateTime::now_utc().year() % 100) as u8;
    if current_year < *utc_time.year.as_ref() {
        formatted.push_str("19");
    } else {
        formatted.push_str("20");
    }
    formatted.push_str(&format!("{:02}", utc_time.year.as_ref()));

    formatted.push('-');
    formatted.push_str(&format!("{:02}", utc_time.month.as_ref()));
    formatted.push('-');
    formatted.push_str(&format!("{:02}", utc_time.day.as_ref()));

    formatted.push(' ');
    formatted.push_str(&format!("{:02}", utc_time.hour.as_ref()));
    formatted.push(':');
    formatted.push_str(&format!("{:02}", utc_time.minute.as_ref()));
    if let Some(second) = utc_time.second.as_ref() {
        formatted.push(':');
        formatted.push_str(&format!("{:02}", second.as_ref()));
    }

    formatted.push_str(" UTC");

    formatted
}

fn format_generalized_time(generalized_time: &GeneralizedTime) -> String {
    let mut formatted = String::new();

    formatted.push_str(&format!("{:04}", generalized_time.year.as_ref()));

    formatted.push('-');
    formatted.push_str(&format!("{:02}", generalized_time.month.as_ref()));
    formatted.push('-');
    formatted.push_str(&format!("{:02}", generalized_time.day.as_ref()));

    formatted.push(' ');
    formatted.push_str(&format!("{:02}", generalized_time.hour.as_ref()));
    formatted.push(':');
    formatted.push_str(&format!("{:02}", generalized_time.minute.as_ref()));
    formatted.push(':');
    formatted.push_str(&format!("{:02}", generalized_time.second.as_ref()));

    if let Some(local_time) = generalized_time.local_time.as_ref() {
        formatted.push(' ');
        formatted.push(local_time.time_direction.into());
        formatted.push_str(&format!("{:02}", local_time.hour.as_ref()));
        formatted.push(':');
        formatted.push_str(&format!("{:02}", local_time.minute.as_ref()));
    } else {
        formatted.push_str(" UTC");
    }

    formatted
}
