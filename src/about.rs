use yew::{classes, function_component, html, Html};

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class={classes!("vertical", "about-container")}>
            <span>{"Crypto-helper"}</span>
            <span>{"The crypto-helper is an online app that helps to work with the diferent crypto algorithms:"}</span>
            <vl>
                <li>{"MD5"}</li>
                <li>{"SHA1/SHA256/SHA512"}</li>
                <li>{"Kerberos ciphers: AES128-CTS-HMAC-SHA1-96/AES256-CTS-HMAC-SHA1-96"}</li>
                <li>{"Kerberos HMAC: HMAC-SHA1-96-AES128/HMAC-SHA1-96-AES256"}</li>
                <li>{"RSA"}</li>
                <li class={classes!("coming-soon")}>{"Coming soon: JWT/JWE debugger."}</li>
            </vl>
            <span>{"All computations perform on the client side. This tool never sends the data the any servers. Tip: if your input is not hex-encoded then you can use a "}<a href={"https://github.com/TheBestTvarynka/byte-formatter"}>{"byte-formatter"}</a>{" to transform input to the hex format."}</span>
            <span>{"Authors: "}<a href={"https://github.com/TheBestTvarynka"}>{"Pavlo Myroniuk (@TheBestTvarynka)"}</a></span>
            <span>{"GitHub: "}<a href={"https://github.com/TheBestTvarynka/crypto-helper"}>{"https://github.com/TheBestTvarynka/crypto-helper"}</a></span>
        </div>
    }
}
