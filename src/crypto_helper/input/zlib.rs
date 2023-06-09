use yew::{function_component, html, Callback, Html, Properties};

use crate::crypto_helper::algorithm::ZlibInput as ZlibInputData;

#[derive(PartialEq, Properties, Clone)]
pub struct ZlibInputProps {
    pub input: ZlibInputData,
    pub input_setter: Callback<ZlibInputData>,
}

#[function_component(ZlibInput)]
pub fn zlib_input(props: &ZlibInputProps) -> Html {
    html! {
        <div>
        </div>
    }
}

pub fn build_zlib_input(input: ZlibInputData, input_setter: Callback<ZlibInputData>) -> Html {
    html! {
        <ZlibInput {input} {input_setter} />
    }
}
