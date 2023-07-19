use yew::{function_component, html, Html};
use yew_notifications::{Notification, NotificationFactory, NotificationsProvider};
use yew_router::{BrowserRouter, Switch};

use crate::components::{footer, Header};
use crate::router::Route;
use crate::routes;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <routes::CryptoHelper /> },
        Route::CryptoHelper => html! { <routes::CryptoHelper /> },
        Route::Jwt => html! { <routes::Jwt /> },
        Route::About => html! { <routes::About /> },
        Route::NotFound => routes::not_found(),
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let component_creator = NotificationFactory;

    html! {
        <BrowserRouter>
            <NotificationsProvider<Notification, NotificationFactory> {component_creator}>
                <div class="body">
                    <Header />
                    <Switch<Route> render={switch} />
                    {footer()}
                </div>
            </NotificationsProvider<Notification, NotificationFactory>>
        </BrowserRouter>
    }
}
