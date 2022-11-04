use yew::{Component, html};

pub struct Footer;

impl Component for Footer {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Footer
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html!{
            <footer>{"footer"}</footer>
        }
    }
}
