use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use similar::capture_diff_slices;
use wasm_bindgen::{JsCast, JsValue};
use yew_agent::oneshot::oneshot;
use yew_agent::Codec;

use crate::diff::diff_algo::DiffAlgo;
use crate::diff::DiffData;

/// Codes for messages encoding/decoding between main thread and worker.
///
/// We are using the custom codec because default `Bincode` fails to decode [DiffData].
pub struct JsonCodec;

impl Codec for JsonCodec {
    fn encode<I>(input: I) -> JsValue
    where
        I: Serialize,
    {
        let encoded = serde_json::to_string(&input).expect("Json serialization should not fail");
        JsValue::from(Uint8Array::from(encoded.as_bytes()))
    }

    fn decode<O>(input: JsValue) -> O
    where
        O: for<'de> Deserialize<'de>,
    {
        let encoded = input.dyn_into::<Uint8Array>().expect("JsValue should be Uint8Array");
        serde_json::from_slice(&encoded.to_vec()).expect("Json deserialization should not fail")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct DiffTaskParams {
    pub algo: DiffAlgo,
    pub original: Vec<char>,
    pub changed: Vec<char>,
}

#[oneshot]
pub async fn DiffTask(params: DiffTaskParams) -> DiffData {
    let DiffTaskParams {
        algo,
        original,
        changed,
    } = params;

    let changes = capture_diff_slices(algo.into(), &original, &changed);

    DiffData {
        original,
        changed,
        changes,
    }
}
