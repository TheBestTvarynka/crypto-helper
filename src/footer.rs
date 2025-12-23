use yew::{Html, html};

pub fn footer() -> Html {
    html! {
        <footer>
            <span>{"Crypto helper Copyright © 2023-2026 Pavlo Myroniuk; released as "}
                <a href="https://github.com/TheBestTvarynka/crypto-helper">{"open source"}</a>{" under the "}
                <a href="https://github.com/TheBestTvarynka/crypto-helper/blob/main/LICENSE">{"MIT"}</a>{" license."}
            </span>
            <span>{"Icons by: "}<a href="https://icons8.com">{"icons8.com"}</a></span>
        </footer>
    }
}
