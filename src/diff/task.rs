use serde::{Deserialize, Serialize};
use similar::capture_diff_slices;
use yew_agent::oneshot::oneshot;

use crate::diff::diff_algo::DiffAlgo;
use crate::diff::DiffData;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct DiffTaskParams {
    pub algo: DiffAlgo,
    pub original: Vec<char>,
    pub changed: Vec<char>,
}

#[oneshot]
pub async fn DiffTask(params: DiffTaskParams) -> DiffData {
    debug!("inside of the task: {:?}", params);

    let DiffTaskParams {
        algo,
        original,
        changed,
    } = params;

    let changes = capture_diff_slices(algo.into(), &original, &changed);

    debug!("changes here");

    DiffData {
        original,
        changed,
        changes,
    }
}
