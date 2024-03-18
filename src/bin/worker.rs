use crypto_helper::diff::{DiffTask, JsonCodec};
use yew_agent::Registrable;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    DiffTask::registrar().encoding::<JsonCodec>().register();
}
