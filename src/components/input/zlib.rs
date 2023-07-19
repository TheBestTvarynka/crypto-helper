use yew::{function_component, html, Callback, Html, Properties};

use crate::components::byte_input::build_byte_input;
use crate::components::Switch;
use crate::crypto_helper::algorithm::ZlibInput as ZlibInputData;

#[derive(PartialEq, Properties, Clone)]
pub struct ZlibInputProps {
    pub input: ZlibInputData,
    pub input_setter: Callback<ZlibInputData>,
}

#[function_component(ZlibInput)]
pub fn zlib_input(props: &ZlibInputProps) -> Html {
    let ZlibInputProps { input, input_setter } = props.clone();
    let ZlibInputData { mode, data } = input;

    let set_input = input_setter.clone();
    let zlib_data_setter = Callback::from(move |data: Vec<u8>| {
        set_input.emit(ZlibInputData { mode, data });
    });

    let zlib_data = data.clone();
    let set_mode = Callback::from(move |mode: bool| {
        input_setter.emit(ZlibInputData {
            mode: mode.into(),
            data: zlib_data.clone(),
        });
    });

    html! {
        <div class="vertical">
            <div class="horizontal">
                <span class="total">{"compress"}</span>
                <Switch id={"zlib-mode".to_string()} setter={set_mode} state={bool::from(mode)}/>
                <span class="total">{"decompress"}</span>
            </div>
            {build_byte_input(data.clone(), zlib_data_setter, None, Some("zlib".into()))}
        </div>
    }
}

pub fn build_zlib_input(input: ZlibInputData, input_setter: Callback<ZlibInputData>) -> Html {
    html! {
        <ZlibInput {input} {input_setter} />
    }
}
