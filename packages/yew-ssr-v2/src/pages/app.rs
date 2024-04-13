use yew::prelude::*;
use std::rc::Rc;

use crate::types::types::*;

use crate::Action;
use crate::StateContext;

use crate::components::app_view::AppView;


#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

#[cfg(feature = "ssr")]
async fn fetch_app(id: String) -> AppType {
    let target_url = format!("https://balticlsc.iem.pw.edu.pl/backend/app?appUid={}", id);
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:3000/")
        .header("Content-Type", "application/json")
        .header("Target-URL", &target_url)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let json_response: serde_json::Value = response.json().await.unwrap();

        if let Some(data) = json_response.get("data") {
            let fetched_app: AppType = serde_json::from_value(data.clone()).unwrap();
            return fetched_app;
        } else {
            AppType {
                shortDescription: None,
                longDescription: None,
                icon: "".to_string(),
                releases: Vec::new(),
                inCockpit: None,
                isApp: None,
                isService: None,
                name: "".to_string(),
                uid: "".to_string(),
            }
        }
    } else {
        AppType {
            shortDescription: None,
            longDescription: None,
            icon: "".to_string(),
            releases: Vec::new(),
            inCockpit: None,
            isApp: None,
            isService: None,
            name: "".to_string(),
            uid: "".to_string(),
        }
    }
}

#[function_component]
fn Content(props: &Props) -> HtmlResult {
    let state_ctx = use_context::<StateContext>().unwrap();
    let state = state_ctx.clone();
    
    let _props_id = props.id.clone();

    let app_rc = use_prepared_state!(async move |_| -> AppType { fetch_app(_props_id).await }, ())?.unwrap();
    let app = Rc::try_unwrap(app_rc).unwrap_or_else(|rc| (*rc).clone());

    state.dispatch(Action::UpdateApp(app.clone()));

    Ok(html! {
        <AppView />
    })
}


#[function_component(App)]
pub fn app(props: &Props) -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};
    let props_id = props.id.clone();

    html! {
        <Suspense {fallback}>
            <Content id={props_id} />
        </Suspense>
    }
}
