use asn1_parser::{OwnedRawAsn1EntityData, UtcTime};
use yew::{function_component, html, Html, Properties};

use crate::asn1::node_options::NodeOptions;

#[derive(PartialEq, Properties, Clone)]
pub struct UtcTimeNodeProps {
    pub node: UtcTime,
    pub meta: OwnedRawAsn1EntityData,
}

#[function_component(UtcTimeNode)]
pub fn utc_time_string(props: &UtcTimeNodeProps) -> Html {
    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    html! {
        <div class="terminal-asn1-node">
            <NodeOptions node_bytes={props.meta.raw_bytes().to_vec()} {offset} {length_len} {data_len} name={String::from("UtcTime")}/>
            <span class="asn-simple-value">{format_utc_time(&props.node)}</span>
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
