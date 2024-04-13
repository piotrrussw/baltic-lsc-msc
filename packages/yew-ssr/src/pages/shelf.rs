use yew::prelude::*;
use std::rc::Rc;

use crate::types::types::*;

use crate::Action;
use crate::StateContext;

use crate::components::data_shelf_list::DataShelfList;

#[cfg(feature = "ssr")]
async fn fetch_data_shelf() -> Vec<DataShelfType> {
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:3000/")
        .header("Content-Type", "application/json")
        .header("Target-URL", "https://balticlsc.iem.pw.edu.pl/backend/task/dataShelf")
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let json_response: serde_json::Value = response.json().await.unwrap();

        if let Some(data) = json_response.get("data") {
            let fetched_data_shelf: Vec<DataShelfType> = serde_json::from_value(data.clone()).unwrap();
            return fetched_data_shelf;
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

#[function_component]
fn Content() -> HtmlResult {
    let state_ctx = use_context::<StateContext>().unwrap();
    let state = state_ctx.clone();

    let data_shelf_rc = use_prepared_state!(async move |_| -> Vec<DataShelfType> { fetch_data_shelf().await }, ())?.unwrap();
    let data_shelf = Rc::try_unwrap(data_shelf_rc).unwrap_or_else(|rc| (*rc).clone());

    state.dispatch(Action::UpdateDataShelf(data_shelf.clone())); 

    Ok(html! {
        <DataShelfList />
    })
}

#[function_component(Shelf)]
pub fn shelf() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}
