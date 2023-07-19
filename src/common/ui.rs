use web_sys::MouseEvent;
use yew::{classes, Callback, Classes, UseStateSetter};

use super::BytesFormat;

pub fn get_format_button_class(selected: bool) -> Classes {
    if selected {
        classes!("format-button", "format-button-selected")
    } else {
        classes!("format-button")
    }
}

pub fn get_set_format_callback(format: BytesFormat, set_format: UseStateSetter<BytesFormat>) -> Callback<MouseEvent> {
    Callback::from(move |_event| {
        set_format.set(format);
    })
}
