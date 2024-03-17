use serde::{Deserialize, Serialize};
use similar::capture_diff_slices;
use yew_agent::oneshot::oneshot;

use crate::diff::diff_algo::DiffAlgo;
use crate::diff::DiffData;

#[derive(Serialize, Deserialize)]
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
