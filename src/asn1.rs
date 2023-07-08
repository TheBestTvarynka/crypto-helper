mod asn1_viewer;

use asn1_parser::Asn1;
use web_sys::KeyboardEvent;
use yew::{function_component, html, use_state, Callback, Html, classes};

use crate::{common::{ByteInput, Checkbox}, asn1::asn1_viewer::Asn1Viewer};

const TEST_ASN1: &[u8] = &[
    48, 50, 161, 17, 12, 15, 116, 104, 101, 98, 101, 115, 116, 116, 118, 97, 114, 121, 110, 107, 97, 162, 9,
    12, 7, 113, 107, 97, 116, 105, 111, 110, 163, 18, 4, 16, 252, 179, 92, 152, 40, 255, 170, 90, 80, 236, 156,
    221, 80, 86, 181, 110,
];

#[function_component(Asn1ParserPage)]
pub fn asn1_parser_page() -> Html {
    let auto_decode = use_state(|| true);
    let raw_asn1 = use_state(|| TEST_ASN1.to_vec());
    let parsed_asn1 = use_state(|| Asn1::default());

    let set_auto_decode = auto_decode.setter();
    let set_checked = Callback::from(move |checked| {
        set_auto_decode.set(checked);
    });

    let parse_asn1 = Callback::from(move |_: ()| {
        //
    });
    let go = parse_asn1.clone();
    let onclick = Callback::from(move |_| {
        parse_asn1.emit(());
    });

    let onkeydown = Callback::from(move |event: KeyboardEvent| {
        if event.ctrl_key() && event.code() == "Enter" {
            go.emit(());
        }
    });

    let raw_asn1_setter = raw_asn1.setter();

    html! {
        <div class={classes!("vertical", "asn1-page")} {onkeydown}>
            <ByteInput bytes={(*raw_asn1).clone()} setter={Callback::from(move |data| raw_asn1_setter.set(data))} placeholder={"asn1 data".to_owned()} />
            <div class="horizontal">
                <button class="action-button" {onclick}>{"Process"}</button>
                <Checkbox id={"auto-decode-asn1".to_owned()} name={"auto-decode".to_owned()} checked={*auto_decode} {set_checked} />
            </div>
            <Asn1Viewer data={(*raw_asn1).clone()} structure={(*parsed_asn1).clone()} />
        </div>
    }
}
