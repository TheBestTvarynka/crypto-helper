use yew::{classes, function_component, html, Html};

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class={classes!("vertical", "about-container")}>
            <span>{"Crypto-helper"}</span>
            <span>{"The crypto-helper is an online app that helps to work with the diferent crypto algorithms:"}</span>
            <ul>
                <li>{"MD5"}</li>
                <li>{"SHA1/SHA256/SHA512"}</li>
                <li>{"Kerberos ciphers: AES128-CTS-HMAC-SHA1-96/AES256-CTS-HMAC-SHA1-96"}</li>
                <li>{"Kerberos HMAC: HMAC-SHA1-96-AES128/HMAC-SHA1-96-AES256"}</li>
                <li>{"RSA"}</li>
                <li>{"JWT debugger. Supported signature algorithms:"}</li>
                <ul>
                    <li>{"none"}</li>
                    <li>{"HS256"}</li>
                    <li>{"HS384"}</li>
                    <li>{"HS512"}</li>
                    <li>{"RS256"}</li>
                    <li>{"RS384"}</li>
                </ul>
            </ul>
            <span>{"All computations are performed on the client side. This tool never sends the data the any servers. Tip: if your input is not hex-encoded then you can use a "}<a href={"https://bf.qkation.com"}>{"byte-formatter"}</a>{" to transform input to the hex format."}</span>
            <span>{"Authors: "}<a href={"https://github.com/TheBestTvarynka"}>{"Pavlo Myroniuk (@TheBestTvarynka)"}</a></span>
            <span>{"GitHub: "}<a href={"https://github.com/TheBestTvarynka/crypto-helper"}>{"https://github.com/TheBestTvarynka/crypto-helper"}</a></span>
        </div>
    }
}
