use yew::prelude::*;
use std::rc::Rc;

use serde_json::json;

use crate::components::apps_list::AppsList;
use crate::components::filters::Filters;

use crate::types::types::*;

use crate::Action;
use crate::StateContext;


#[cfg(feature = "ssr")]
async fn fetch_apps() -> Vec<AppType> {
    let payload = json!({
        "onlyApps": true,
        "onlyLastRelease": false
    });

    let client = reqwest::Client::new();
    let response = client.post("http://localhost:3000/")
        .header("Content-Type", "application/json")
        .header("Target-URL", "https://balticlsc.iem.pw.edu.pl/backend/app/list")
        .body(payload.to_string())
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let json_response: serde_json::Value = response.json().await.unwrap();

        if let Some(data) = json_response.get("data") {
            let fetched_apps: Vec<AppType> = serde_json::from_value(data.clone()).unwrap();
            return fetched_apps;
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

#[cfg(feature = "ssr")]
async fn fetch_shelf() -> Vec<ShelfType> {
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:3000/")
        .header("Content-Type", "application/json")
        .header("Target-URL", "https://balticlsc.iem.pw.edu.pl/backend/app/shelf")
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let json_response: serde_json::Value = response.json().await.unwrap();

        if let Some(data) = json_response.get("data") {
            let fetched_shelf: Vec<ShelfType> = serde_json::from_value(data.clone()).unwrap();
            return fetched_shelf;
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

    let apps_rc = use_prepared_state!(async move |_| -> Vec<AppType> { fetch_apps().await }, ())?.unwrap();
    let apps = Rc::try_unwrap(apps_rc).unwrap_or_else(|rc| (*rc).clone());

    state.dispatch(Action::UpdateApps(apps));

    let shelf_rc = use_prepared_state!(async move |_| -> Vec<ShelfType> { fetch_shelf().await }, ())?.unwrap();
    let shelf = Rc::try_unwrap(shelf_rc).unwrap_or_else(|rc| (*rc).clone());

    state.dispatch(Action::UpdateShelf(shelf)); 
    
    Ok(html! {
        <main>
            <Filters />
            <AppsList  />
        </main>
    })
}

#[function_component(Store)]
pub fn store() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}

