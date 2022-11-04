use yew::{Component, html};

pub struct Header;

impl Component for Header {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Header
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html!{
            <header>{"header"}</header>
        }
    }
}
