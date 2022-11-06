use yew::{html, Component};

pub struct Footer;

impl Component for Footer {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Footer
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <footer>
                <span>{"Crypto helper Copyright © 2022 Pavlo Myroniuk; released as "}
                    <a href="https://github.com/TheBestTvarynka/crypto-helper">{"open source"}</a>{" under the "}
                    <a href="https://github.com/TheBestTvarynka/crypto-helper/blob/main/LICENSE">{"MIT"}</a>{" license."}
                </span>
                <span>{"Icons by: "}<a href="https://icons8.com">{"icons8.com"}</a></span>
            </footer>
        }
    }
}
