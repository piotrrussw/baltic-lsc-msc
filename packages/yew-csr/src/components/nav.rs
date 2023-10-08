use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

pub struct Nav;

impl Component for Nav {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <nav>
            <div class="nav-container">
              <div class="nav-logo">
                <Link<Route> to={Route::StoreRoute}>
                <div class="row">
                  <div class="col col-sm-6">{"Baltic LSC"}</div>
                  <div class="col col-sm-6">
                    <img src="assets/logo.jpg" width={150} height="100%" />
                  </div>
                </div>
              </Link<Route>>
              </div>
              <ul class="nav-links">
                <li>
                  <Link<Route> to={Route::StoreRoute}>{"Store"}</Link<Route>>
                </li>
                <li>
                  <Link<Route> to={Route::ShelfRoute}>{"Shelf"}</Link<Route>>
                </li>
                // <li>
                //   <Link<Route> to={Route::AppRoute {id: 123}}>{"App"}</Link<Route>>
                // </li>
              </ul>
            </div>
        </nav>
        }
    }
}
