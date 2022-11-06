use yew::{classes, html, Html};

pub fn build_simple_output(output: &[u8]) -> Html {
    html! {
        <div class={classes!("output")}>
            <span class={classes!("simple-digest")}>{hex::encode(output)}</span>
            <span class={classes!("total")}>{format!("total: {}", output.len())}</span>
        </div>
    }
}
