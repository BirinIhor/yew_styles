use yew::prelude::*;
use yew_router::{prelude::*, Switch, switch::Permissive , route::Route};
use page::ButtonPage;

pub struct App;

#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to= "/!"]
    RootPath,
    #[to= "/button!"]
    ButtonPath,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Router<AppRouter, ()>
                    render = Router::render(|switch: AppRouter | {
                        match switch {
                            AppRouter::RootPath => html!{
                                <div>
                                  <h2>{"Yew Styles Component"}</h2>
                                  <div>
                                    <a href="/button">{"Button"}</a>
                                  </div>
                                </div>
                            },
                            AppRouter::ButtonPath => html!{<ButtonPage/>},
                            AppRouter::PageNotFound(Permissive(None)) => html!{"Page not found"},
                            AppRouter::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                        }
                    } )
                    redirect = Router::redirect(|route: Route<()>| {
                        AppRouter::PageNotFound(Permissive(Some(route.route)))
                    })
                />
            </div>
        }
    }
 }