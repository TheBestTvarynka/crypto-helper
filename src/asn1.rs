#[macro_use]
mod macros;

mod asn1_viewer;
mod hex_view;
mod node_options;
mod scheme;

use std::rc::Rc;

use asn1_parser::{Asn1, Asn1Decoder, Asn1Encoder};
use web_sys::KeyboardEvent;
use yew::{classes, function_component, html, use_effect_with_deps, use_reducer, use_state, Callback, Html, Reducible};
use yew_hooks::{use_clipboard, use_local_storage, use_location};
use yew_notifications::{use_notification, Notification, NotificationType};

use crate::asn1::asn1_viewer::Asn1Viewer;
use crate::asn1::hex_view::HexViewer;
use crate::common::{encode_bytes, ByteInput, BytesFormat};
use crate::url_query_params;
use crate::url_query_params::generate_asn1_link;

pub const TEST_ASN1: &[u8] = &[
    48, 87, 1, 1, 255, 1, 1, 0, 160, 17, 12, 15, 84, 98, 101, 66, 101, 115, 116, 84, 118, 97, 114, 121, 110, 107, 97,
    161, 60, 48, 58, 5, 0, 164, 9, 4, 7, 48, 5, 160, 3, 1, 1, 255, 164, 7, 3, 5, 0, 64, 129, 0, 16, 164, 34, 108, 32,
    48, 30, 160, 2, 5, 0, 161, 24, 30, 22, 0, 67, 0, 101, 0, 114, 0, 116, 0, 105, 0, 102, 0, 105, 0, 99, 0, 97, 0, 116,
    0, 101,
];
const ASN1_LOCAL_STORAGE_KEY: &str = "ASN1_DATA";

pub fn compare_ids(asn1_node_id: u64, cur_node: &Option<u64>) -> bool {
    matches!(cur_node, Some(node_id) if *node_id == asn1_node_id)
}

pub enum HighlightAction {
    Show(u64),
    Hide(u64),
}

#[derive(Debug, Default, Clone)]
pub struct Highlight {
    nodes: Vec<u64>,
}

impl Highlight {
    fn show(&mut self, id: u64) {
        self.hide(id);
        self.nodes.push(id);
    }

    fn hide(&mut self, id: u64) {
        while let Some(index) = self.nodes.iter().position(|asn1_id| *asn1_id == id) {
            self.nodes.remove(index);
        }
    }

    fn current(&self) -> Option<u64> {
        self.nodes.last().copied()
    }
}

impl Reducible for Highlight {
    type Action = HighlightAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut highlight = self.as_ref().clone();
        match action {
            HighlightAction::Show(id) => highlight.show(id),
            HighlightAction::Hide(id) => highlight.hide(id),
        }
        Rc::new(highlight)
    }
}

#[function_component(Asn1ParserPage)]
pub fn asn1_parser_page() -> Html {
    let notification_manager = use_notification::<Notification>();

    let raw_asn1 = use_state(|| TEST_ASN1.to_vec());
    let parsed_asn1 = use_state(|| Asn1::decode_buff(TEST_ASN1).unwrap());

    let notifications = use_notification::<Notification>();
    let asn1_setter = parsed_asn1.setter();
    let raw_data = (*raw_asn1).clone();
    let parse_asn1 = Callback::from(move |_| match Asn1::decode_buff(&raw_data) {
        Ok(asn1) => asn1_setter.set(asn1.to_owned_with_asn1(asn1.inner_asn1().to_owned())),
        Err(error) => notifications.spawn(Notification::new(
            NotificationType::Error,
            "Invalid asn1 data",
            error.message(),
            Notification::NOTIFICATION_LIFETIME,
        )),
    });

    let process = parse_asn1.clone();
    let onkeydown = Callback::from(move |event: KeyboardEvent| {
        if event.ctrl_key() && event.code() == "Enter" {
            process.emit(());
        }
    });

    let onclick = Callback::from(move |_| {
        parse_asn1.emit(());
    });

    let location = use_location();
    let notifications = notification_manager.clone();
    let raw_asn1_setter = raw_asn1.setter();
    let asn1_setter = parsed_asn1.setter();
    let local_storage = use_local_storage::<String>(ASN1_LOCAL_STORAGE_KEY.to_owned());
    use_effect_with_deps(
        move |_: &[(); 0]| {
            let query = &location.search;

            if query.len() < 2 {
                // URL query params is empty. We try to load ASN1 from local storage.
                if let Some(raw_asn1) = (*local_storage).as_ref() {
                    if let Ok(bytes) = hex::decode(raw_asn1) {
                        match Asn1::decode_buff(&bytes) {
                            Ok(asn1) => {
                                asn1_setter.set(asn1.to_owned_with_asn1(asn1.inner_asn1().to_owned()));
                            }
                            Err(err) => {
                                error!("Can not decode asn1: {:?}", err);
                            }
                        }
                        raw_asn1_setter.set(bytes);
                    }
                }
                return;
            }

            match serde_qs::from_str(&query[1..]) {
                Ok(asn1) => {
                    let url_query_params::Asn1 { asn1: asn1_data } = asn1;
                    match Asn1::decode_buff(&asn1_data) {
                        Ok(asn1) => {
                            asn1_setter.set(asn1.to_owned_with_asn1(asn1.inner_asn1().to_owned()));
                        }
                        Err(error) => notifications.spawn(Notification::new(
                            NotificationType::Error,
                            "Invalid asn1 data",
                            error.message(),
                            Notification::NOTIFICATION_LIFETIME,
                        )),
                    };
                    raw_asn1_setter.set(asn1_data);
                }
                Err(err) => notifications.spawn(Notification::new(
                    NotificationType::Error,
                    "Can not load data from url",
                    err.to_string(),
                    Notification::NOTIFICATION_LIFETIME,
                )),
            }
        },
        [],
    );

    let local_storage = use_local_storage::<String>(ASN1_LOCAL_STORAGE_KEY.to_owned());
    use_effect_with_deps(
        move |asn1| {
            let mut encoded = vec![0; asn1.needed_buf_size()];
            asn1.encode_buff(&mut encoded).expect("ASN1 encoding should not fail");
            local_storage.set(encode_bytes(encoded, BytesFormat::Hex));
        },
        parsed_asn1.clone(),
    );

    let clipboard = use_clipboard();
    let raw_asn1_data = (*raw_asn1).clone();
    let share_by_link = Callback::from(move |_| {
        clipboard.write_text(generate_asn1_link(raw_asn1_data.clone()));

        notification_manager.spawn(Notification::from_description_and_type(
            NotificationType::Info,
            "link copied",
        ));
    });

    let raw_asn1_setter = raw_asn1.setter();

    let ctx = use_reducer(Highlight::default);
    let asn1_dispatcher = ctx.dispatcher();
    let hex_dispatcher = ctx.dispatcher();

    html! {
        <div class={classes!("vertical", "asn1-page")} {onkeydown}>
            <span>
                {"Still in β (beta). See "}
                    <a href="https://github.com/TheBestTvarynka/crypto-helper/tree/main/crates/asn1-parser#supported-asn1-types" class="a-link">
                        {"list of supported asn1 types"}
                    </a>
                {". Report a bug/feature "}<a href="https://github.com/TheBestTvarynka/crypto-helper/issues/new" class="a-link">{"here"}</a>{"."}
            </span>
            <ByteInput bytes={(*raw_asn1).clone()} setter={Callback::from(move |data| raw_asn1_setter.set(data))} placeholder={"asn1 data".to_owned()} rows={10} />
            <div class="horizontal">
                <button class="action-button" {onclick}>{"Decode"}</button>
                <span class="total">{"(ctrl+enter)"}</span>
                <button class="button-with-icon" onclick={share_by_link}>
                    <img src="/public/img/icons/share_by_link.png" />
                </button>
            </div>
            <div class="asn1-viewers">
                <Asn1Viewer
                    structure={(*parsed_asn1).clone()}
                    cur_node={(*ctx).current()}
                    set_cur_node={move |action| asn1_dispatcher.dispatch(action)}
                />
                <HexViewer
                    structure={(*parsed_asn1).clone()}
                    cur_node={(*ctx).current()}
                    set_cur_node={move |action| hex_dispatcher.dispatch(action)}
                />
            </div>
        </div>
    }
}
