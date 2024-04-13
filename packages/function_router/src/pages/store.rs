use yew::prelude::*;
use gloo_net::http::Request;
use serde_json::json;

use crate::components::apps_list::AppsList;
use crate::components::filters::Filters;

use crate::types::types::*;

use crate::Action;
use crate::StateContext;


#[function_component(Store)]
pub fn store() -> Html {
    let state_ctx = use_context::<StateContext>().unwrap();

    {
        let state_ctx_clone = state_ctx.clone(); // Clone state_ctx
        let payload = json!({
            "onlyApps": true,
            "onlyLastRelease": false
        });

        use_effect_with_deps(move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let response: serde_json::Value =
                        Request::post("http://localhost:3000/")
                            .header("Content-Type", "application/json")
                            .header("Target-URL", "https://balticlsc.iem.pw.edu.pl/backend/app/list")
                            .body(payload.to_string())
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();

                if let Some(data) = response.get("data") {
                    let fetched_apps: Vec<AppType> = serde_json::from_value(data.clone()).unwrap();
                    state_ctx_clone.dispatch(Action::UpdateApps(fetched_apps));
                }
            });
            || ()
        }, (),
        );
    }

    {
        let state_ctx_clone = state_ctx.clone(); // Clone state_ctx

        use_effect_with_deps(move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let response: serde_json::Value =
                        Request::get("http://localhost:3000/")
                            .header("Content-Type", "application/json")
                            .header("Target-URL", "https://balticlsc.iem.pw.edu.pl/backend/app/shelf")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();

                if let Some(data) = response.get("data") {
                    let fetched_shelf: Vec<ShelfType> = serde_json::from_value(data.clone()).unwrap();
                    state_ctx_clone.dispatch(Action::UpdateShelf(fetched_shelf));
                }
            });
            || ()
        }, (),
        );
    }

    html! {
        <main>
            <Filters />
            <AppsList  />
        </main>
    }
}

