use std::collections::HashMap;

use std::rc::Rc;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::pages::app::App as AppPage;
use crate::pages::shelf::Shelf;
use crate::pages::store::Store;

use crate::types::types::AppType;
use crate::types::types::DataShelfType;
use crate::types::types::ShelfType;

use crate::components::nav::Nav;

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub sort: String,
    pub search: String,
    pub apps: Vec<AppType>,
    pub shelf: Vec<ShelfType>,
    pub data_shelf: Vec<DataShelfType>,
    pub app: AppType,
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
            html! { <AppPage id={id} /> }
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
            Action::UpdateDataShelf(new_data_shelf) => self.update_data_shelf(new_data_shelf),
            Action::UpdateApp(new_app) => self.update_app(new_app),
        }
    }
}

pub enum Action {
    UpdateSearch(String),
    UpdateSort(String),
    UpdateApps(Vec<AppType>),
    UpdateShelf(Vec<ShelfType>),
    UpdateDataShelf(Vec<DataShelfType>),
    UpdateApp(AppType),
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

    fn update_data_shelf(&self, new_data_shelf: Vec<DataShelfType>) -> Rc<Self> {
        Rc::new(State {
            data_shelf: new_data_shelf,
            ..(*self).clone()
        })
    }

    fn update_app(&self, new_app: AppType) -> Rc<Self> {
        Rc::new(State {
            app: new_app,
            ..(*self).clone()
        })
    }
}

pub type StateContext = UseReducerHandle<State>;

#[function_component]
pub fn App() -> Html {
    let ctx = use_reducer(|| State {
        search: "".to_string(),
        sort: "date".to_string(),
        apps: vec![],
        shelf: vec![],
        data_shelf: vec![],
        app: AppType {
            shortDescription: None,
            longDescription: None,
            icon: "".to_string(),
            releases: Vec::new(),
            inCockpit: None,
            isApp: None,
            isService: None,
            name: "".to_string(),
            uid: "".to_string(),
        },
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

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let ctx = use_reducer(|| State {
        search: "".to_string(),
        sort: "date".to_string(),
        apps: vec![],
        shelf: vec![],
        data_shelf: vec![],
        app: AppType {
            shortDescription: None,
            longDescription: None,
            icon: "".to_string(),
            releases: Vec::new(),
            inCockpit: None,
            isApp: None,
            isService: None,
            name: "".to_string(),
            uid: "".to_string(),
        },
    });

    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <ContextProvider<StateContext> context={ctx}>
            <div>
                <Router history={history}>
                    <Nav/>
                    <main class="container">
                        <Switch<Route> render={switch} />
                    </main>
                    </Router>
            </div>
        </ContextProvider<StateContext>>
    }
}
