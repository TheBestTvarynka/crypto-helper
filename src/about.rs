use yew::{classes, function_component, html, Html};

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class={classes!("vertical", "about-container")}>
            <span>{"Crypto-helper"}</span>
            <span>{"The crypto-helper is an online app that helps to work with the diferent crypto algorithms:"}</span>
            <ul>
                <li>{"Argon2"}</li>
                <li>{"BCrypt"}</li>
                <li>{"MD5"}</li>
                <li>{"Argon2"}</li>
                <li>{"BCRYPT"}</li>
                <li>{"SHA1/SHA256/SHA384/SHA512"}</li>
                <li>{"Kerberos ciphers: AES128-CTS-HMAC-SHA1-96/AES256-CTS-HMAC-SHA1-96"}</li>
                <li>{"Kerberos HMAC: HMAC-SHA1-96-AES128/HMAC-SHA1-96-AES256"}</li>
                <li>{"RSA"}</li>
                <li>{"Compression: ZLIB"}</li>
                <li>{"JWT debugger. Supported signature algorithms:"}</li>
                <ul>
                    <li>{"none"}</li>
                    <li>{"HS256"}</li>
                    <li>{"HS384"}</li>
                    <li>{"HS512"}</li>
                    <li>{"RS256"}</li>
                    <li>{"RS384"}</li>
                    <li>{"RS512"}</li>
                    <li>{"ES256"}</li>
                    <li>{"ES384"}</li>
                    <li>{"ES512"}</li>
                </ul>
                <li>{"ASN1 decoder"}</li>
                <li>{"Diff checker"}</li>
                <li>{"Ability to share the sample by url"}</li>
            </ul>
            <span>{"All computations are performed on the client side."}</span>
            <ul>
                <li>{"This site does not set or use cookies."}</li>
                <li>{"This site does not store data in the browser to be shared, sent, or sold to third-parties."}</li>
                <li>{"No personal information is shared, sent, or sold to third-parties."}</li>
            </ul>
            <span>{"Authors: "}<a href={"https://github.com/TheBestTvarynka"}>{"Pavlo Myroniuk (@TheBestTvarynka)"}</a></span>
            <span>{"GitHub: "}<a href={"https://github.com/TheBestTvarynka/crypto-helper"}>{"https://github.com/TheBestTvarynka/crypto-helper"}</a></span>
        </div>
    }
}
