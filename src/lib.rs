mod routes;

use std::collections::HashMap;



use routes::{
    home::Home,
    my::My,
};

use yew::{function_component, html, AttrValue, Html, Properties};
use yew_router::{Router, history::{AnyHistory, History, MemoryHistory}, BrowserRouter, Routable, Switch};


#[derive(Routable, Clone, PartialEq, Eq, Copy)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/my")]
    My,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::My => html! { <My /> }
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub uri: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.uri, &props.queries)
        .unwrap();
    
    html! {
        <Router history={history}>
            <Switch<Route> render={switch} />
        </Router>
    }
}