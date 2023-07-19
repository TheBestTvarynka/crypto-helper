use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/crypto-helper")]
    CryptoHelper,
    #[at("/jwt")]
    Jwt,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}
