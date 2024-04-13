// use serde::{Deserialize, Serialize};

// use yew::prelude::*;
// use gloo_net::http::Request;

// use crate::types::types::*;

// use crate::Action;
// use crate::StateContext;

// use crate::components::data_shelf_list::DataShelfList;

// #[cfg(feature = "ssr")]
// async fn fetch_data_shelf(state: StateContext) -> Vec<DataShelfType> {
//     let client = reqwest::Client::new();
//     let response = client.get("http://localhost:3000/")
//         .header("Content-Type", "application/json")
//         .header("Target-URL", "https://balticlsc.iem.pw.edu.pl/backend/task/dataShelf")
//         .send()
//         .await
//         .unwrap();

//     if response.status().is_success() {
//         let body = response.bytes().await.unwrap();
//         let data_shelf: Vec<DataShelfType> = serde_json::from_slice(&body).unwrap();

//         state.dispatch(Action::UpdateDataShelf(data_shelf.clone()));

//         data_shelf
//     } else {
//         Vec::new()
//     }
// }

// #[cfg(feature = "ssr")]
// async fn fetch_uuid() -> Uuid {
//     // reqwest works for both non-wasm and wasm targets.
//     let resp = reqwest::get("https://httpbin.org/uuid").await.unwrap();
//     let uuid_resp = resp.json::<UuidResponse>().await.unwrap();

//     uuid_resp.uuid
// }

// #[function_component]
// fn Content() -> HtmlResult {
//     let uuid = use_prepared_state!((), async move |_| -> Uuid { fetch_uuid().await })?.unwrap();

//     Ok(html! {
//         <div>{"Random UUID: "}{uuid}</div>
//     })
// }


// // #[function_component]
// // fn Content() -> HtmlResult {
// //     let state_ctx = use_context::<StateContext>().unwrap();
// //     let state = state_ctx.clone();

// //     let _data_shelf = fetch_data_shelf(state);

// //     Ok(html! {
// //         <DataShelfList />
// //     })
// // }

// #[function_component(Shelf)]
// pub fn shelf() -> Html {
//     // {
//     //     use_effect_with_deps(move |_| {
//     //             wasm_bindgen_futures::spawn_local(async move {
//     //                 let response: serde_json::Value =
//     //                     Request::get("http://localhost:3000/")
//     //                         .header("Content-Type", "application/json")
//     //                         .header("Target-URL", "https://balticlsc.iem.pw.edu.pl/backend/task/dataShelf")
//     //                         .send()
//     //                         .await
//     //                         .unwrap()
//     //                         .json()
//     //                         .await
//     //                         .unwrap();

//     //             if let Some(data) = response.get("data") {
//     //                 let fetched_data_shelf: Vec<DataShelfType> = serde_json::from_value(data.clone()).unwrap();
//     //                 state.dispatch(Action::UpdateDataShelf(fetched_data_shelf));
//     //             }
//     //         });
//     //         || ()
//     //     }, (),
//     //     );
//     // }

//     let fallback = html! {<div>{"Loading..."}</div>};

//     html! {
//         <Suspense {fallback}>
//             <Content />
//         </Suspense>
//     }
// }


use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct UuidResponse {
    uuid: Uuid,
}

#[cfg(feature = "ssr")]
async fn fetch_uuid() -> Uuid {
    // reqwest works for both non-wasm and wasm targets.
    let resp = reqwest::get("https://httpbin.org/uuid").await.unwrap();
    let uuid_resp = resp.json::<UuidResponse>().await.unwrap();

    uuid_resp.uuid
}

#[function_component]
fn Content() -> HtmlResult {
    let uuid = use_prepared_state!(async move |_| -> Uuid { fetch_uuid().await }, ())?.unwrap();

    Ok(html! {
        <div>{"Random UUID: "}{uuid}</div>
    })
}

#[function_component(Shelf)]
pub fn App() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}
