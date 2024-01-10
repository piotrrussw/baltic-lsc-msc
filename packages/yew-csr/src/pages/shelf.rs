use yew::prelude::*;
use gloo_net::http::Request;

use crate::types::types::*;

use crate::Action;
use crate::StateContext;

use crate::components::data_shelf_list::DataShelfList;

#[function_component(Shelf)]
pub fn shelf() -> Html {
    let state_ctx = use_context::<StateContext>().unwrap();
    let state = state_ctx.clone(); // Clone state_ctx

    {
        use_effect_with_deps(move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let response: serde_json::Value =
                        Request::get("http://localhost:3000/")
                            .header("Content-Type", "application/json")
                            .header("Target-URL", "https://balticlsc.iem.pw.edu.pl/backend/task/dataShelf")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();

                if let Some(data) = response.get("data") {
                    let fetched_data_shelf: Vec<DataShelfType> = serde_json::from_value(data.clone()).unwrap();
                    state.dispatch(Action::UpdateDataShelf(fetched_data_shelf));
                }
            });
            || ()
        }, (),
        );
    }

    html! {
        <DataShelfList />
    }
}

