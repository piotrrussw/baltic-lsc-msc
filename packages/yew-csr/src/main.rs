mod components;
mod pages;
mod types;

use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

use pages::app::App;
use pages::shelf::Shelf;
use pages::store::Store;

use types::types::AppType;
use types::types::ShelfType;

use components::nav::Nav;

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub sort: String,
    pub search: String,
    pub apps: Vec<AppType>,
    pub shelf: Vec<ShelfType>,
}

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    StoreRoute,
    #[at("/shelf")]
    ShelfRoute,
    #[at("/app/:id")]
    AppRoute { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::StoreRoute => {
            html! { <Store /> }
        }
        Route::ShelfRoute => {
            html! { <Shelf /> }
        }
        Route::AppRoute { id } => {
            html! { <App id={id} /> }
        }
        Route::NotFound => {
            html! { <h1>{ "404" }</h1> }
        }
    }
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::UpdateSearch(new_search) => self.update_search(new_search),
            Action::UpdateSort(new_sort) => self.update_sort(new_sort),
            Action::UpdateApps(new_apps) => self.update_apps(new_apps),
            Action::UpdateShelf(new_shelf) => self.update_shelf(new_shelf),
        }
    }
}

pub enum Action {
    UpdateSearch(String),
    UpdateSort(String),
    UpdateApps(Vec<AppType>),
    UpdateShelf(Vec<ShelfType>),
}

impl State {
    fn update_search(&self, new_search: String) -> Rc<Self> {
        Rc::new(State {
            search: new_search,
            ..(*self).clone()
        })
    }

    fn update_sort(&self, new_sort: String) -> Rc<Self> {
        Rc::new(State {
            sort: new_sort,
            ..(*self).clone()
        })
    }

    fn update_apps(&self, new_apps: Vec<AppType>) -> Rc<Self> {
        Rc::new(State {
            apps: new_apps,
            ..(*self).clone()
        })
    }

    fn update_shelf(&self, new_shelf: Vec<ShelfType>) -> Rc<Self> {
        Rc::new(State {
            shelf: new_shelf,
            ..(*self).clone()
        })
    }
}

pub type StateContext = UseReducerHandle<State>;

#[function_component(Main)]
fn app() -> Html {
    let ctx = use_reducer(|| State {
        search: "".to_string(),
        sort: "date".to_string(),
        apps: vec![],
        shelf: vec![],
    });

    html! {
      <ContextProvider<StateContext> context={ctx}>
        <div>
          <BrowserRouter>
          <Nav/>
          <main class="container">
              <Switch<Route> render={switch} />
          </main>
          </BrowserRouter>
        </div>
      </ContextProvider<StateContext>>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
