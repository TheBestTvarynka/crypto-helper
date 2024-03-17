use crypto_helper::diff::DiffTask;
use yew_agent::{Bincode, Registrable};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    DiffTask::registrar().encoding::<Bincode>().register();
}
