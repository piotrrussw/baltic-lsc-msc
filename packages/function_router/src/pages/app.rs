use yew::prelude::*;
use gloo_net::http::Request;

use crate::types::types::*;

use crate::Action;
use crate::StateContext;

use crate::components::app_view::AppView;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

#[function_component(App)]
pub fn app(props: &Props) -> Html {
    let state_ctx = use_context::<StateContext>().unwrap();
    let state = state_ctx.clone(); // Clone state_ctx

    let props_id = props.id.clone();

    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let target_url = format!("https://balticlsc.iem.pw.edu.pl/backend/app?appUid={}", props_id);
                let response: serde_json::Value = Request::get("http://localhost:3000/")
                    .header("Content-Type", "application/json")
                    .header("Target-URL", &target_url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                if let Some(data) = response.get("data") {
                    let app: AppType = serde_json::from_value(data.clone()).unwrap();
                    state.dispatch(Action::UpdateApp(app));
                }
            });

            || ()
        },
        (),
    );

    html! {
        <AppView />
    }
}
