// use serde::Deserialize;
// use serde_json::json;
// use yew::format::Json;
// use gloo_net::http::Request;
// use yew::prelude::*;

// #[derive(Clone, PartialEq, Deserialize)]
// struct BalticApp {
//     name: String,
//     uid: String,
//     p_class: String,
//     short_description: String,
//     long_description: String,
//     icon: String,
//     is_app: bool,
//     is_service: bool,
// }

// #[derive(Properties, PartialEq)]
// struct AppsListProps {
//     apps: Vec<BalticApp>,
// }

// #[function_component(AppsList)]
// fn apps_list(AppsListProps { apps }: &AppsListProps) -> Html {
    // apps.iter()
    //     .map(|it| {
    //         html! {
    //             <li key={it.uid.clone()} class="list-item">
    //                 <div class="list-item_title">
    //                     <div class="list-item_img">
    //                         <img src={it.icon.clone()} />
    //                     </div>
    //                     <h3>{format!("{}", it.name)}</h3>
    //                 </div>
    //                 <p>{format!("{}", it.short_description)}</p>
    //             </li>
    //         }
    //     })
    //     .collect::<Html>()
// }

// #[function_component(App)]
// fn app() -> Html {
    // let apps = use_state(|| vec![]);
    // {
    //     let apps = apps.clone();
    //     use_effect_with_deps(
    //         move |_| {
    //             let apps = apps.clone();
    //             wasm_bindgen_futures::spawn_local(async move {
    //                 let fetched_apps: Vec<BalticApp> =
    //                     Request::post("https://balticlsc.iem.pw.edu.pl/backend/app/list")
    //                         .header("Content-Type", "application/json")
    //                         .body(Json(&json!({"key": "value"})))
    //                         .send()
    //                         .await
    //                         .unwrap()
    //                         .json()
    //                         .await
    //                         .unwrap();
    //                 apps.set(fetched_apps);
    //             });
    //             || ()
    //         },
    //         (),
    //     );
    // }

//     // let apps = vec![
//     //     BalticApp {
//     //         name: "Agris Test".to_string(),
//     //         uid: "23c4c2d2-a637-41d6-b3b8-e60842413d34".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Covid-2 Analyzer".to_string(),
//     //         uid: "441c1cbf-7bf5-4dce-a3c4-fd170d710896".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "Analysis of Covid-2".to_string(),
//     //         long_description: "Analyzes pandemics data, specific to Covid-2".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/cva_001.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Distance Matrix Calculator".to_string(),
//     //         uid: "6d0be02e-4860-45cd-8cde-0d766feefa48".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "Finds all distances between given addresses using user-provided map".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Dtree".to_string(),
//     //         uid: "eb691393-666d-4c8a-bc07-ae4382ca1deb".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Edging Image Processor".to_string(),
//     //         uid: "1f7a053a-ae08-48cf-ae13-ce3c5c9fa4dd".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "Edges color images.".to_string(),
//     //         long_description: "Processes images by splitting into RGB and edging.".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/yap_001.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "FaceAnonimizer".to_string(),
//     //         uid: "89260b61-5526-4146-beee-438a777db515".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "FaceRecognizer".to_string(),
//     //         uid: "a7c69ea2-e110-4e67-8e7f-943059b2e576".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "Recognizes faces in the crowd".to_string(),
//     //         long_description: "Face Recognition Application used to recognize faces in the crowd".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/fcr_001.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Geo CVRP Solver".to_string(),
//     //         uid: "01982b22-b448-42b0-ae56-714ff8d3311f".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "Optimizes waste logistics.".to_string(),
//     //         long_description: "Optimizes the sequence the customers should be visited by multiple vehicles. Takes three inputs:\n1) List of objects with addresses. \n2) Map\n3) List of the same objects with max capacities for vehicles and capacities for costumer demand.".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Greying Image Processor".to_string(),
//     //         uid: "fe1514cb-7edb-49c5-b94a-95b5bdf8549f".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "Greys color images.".to_string(),
//     //         long_description: "Processes images by splitting into RGB and greying.".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/yap_001.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Hull Optimizer".to_string(),
//     //         uid: "3a6710e5-6f6d-446a-9bb1-8290c88e5dc5".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "Optimizes ship hulls.".to_string(),
//     //         long_description: "Ship hull optimization application used to optimize shapes of hulls under various criteria.".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/hlo_001.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Image Classification Trainer".to_string(),
//     //         uid: "da8417db-2302-45db-a779-337d3302e345".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "A neural network model trainer for classification of images.".to_string(),
//     //         long_description: "A neural network model trainer for classification of images. Requires some input data for the process of learning and produces its neural network model to the output. Output model can be used for various predictions, depending on input image data type. ".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/ict_001.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Matrix Multiplication".to_string(),
//     //         uid: "2b7701da-ab50-443c-be66-4b6a3eaa72b5".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Matrix Multiplication Parallel".to_string(),
//     //         uid: "7613694d-ad00-452f-9f65-205c05e5ba41".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "MoodRecognizer".to_string(),
//     //         uid: "08855bb3-1df7-43a2-9f8a-c90d896ee224".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "Recognizes face moods in the crowd".to_string(),
//     //         long_description: "Mood Recognition Application used to recognize face moods in the crowd".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/fcr_001.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Multiple Object Detection".to_string(),
//     //         uid: "ffc6ec85-fed2-45ef-9a42-d6f9f7e91520".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "Detect objects from an Image".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Mushroom Recogniser".to_string(),
//     //         uid: "5f5d89b2-016f-477f-897c-4088c2ea651e".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "data:image/png;base64,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".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "My new demo mobile application".to_string(),
//     //         uid: "59341ec2-e31c-4db9-9d1d-fbcc98e88bde".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "PSO 2D Example".to_string(),
//     //         uid: "32ffec71-8807-43c2-95b7-21cf7d4043b2".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "PW_Test".to_string(),
//     //         uid: "7a146ace-a471-4eed-a7fe-015edb2baf17".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Particle Swarm Optimiser".to_string(),
//     //         uid: "c99d3419-6bcf-4053-972a-fa1eba7ffa72".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Pdf to readable pdf converter".to_string(),
//     //         uid: "18880ad5-e9c3-432a-8c6b-fcb1a0750381".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Photo Colorizer".to_string(),
//     //         uid: "6c071c5f-6f19-4bad-a730-78eb0f7cb5a4".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Power loss test".to_string(),
//     //         uid: "f6a9a7b8-616f-4b14-af47-4e1c0b366e99".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Simple Image Processor".to_string(),
//     //         uid: "18a77860-4447-40ad-a154-3b9ee22fe8da".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "Edges out photos.".to_string(),
//     //         long_description: "Simple image processor that edges out photos.".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/yap_001.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Test An".to_string(),
//     //         uid: "e38b7ae2-e0b3-4bec-9615-09592ab26b68".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Video Anonimizer - single token".to_string(),
//     //         uid: "96b638ca-39e7-443b-95a4-1509d3b7534f".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "Video Splitter".to_string(),
//     //         uid: "fb4c8c77-6070-4474-977c-c7f63bca4f79".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "VideoAnonimizer GridFS".to_string(),
//     //         uid: "1a04e0ab-f505-4a30-ac0d-a84a9f6e46e7".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "VideoAnonimizer combined".to_string(),
//     //         uid: "52baaee1-ef96-4b11-89e2-a2886e6186b4".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "VideoMerger".to_string(),
//     //         uid: "1361b54f-6bc0-4ebb-9fc5-549078b76376".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "VideoUpscaler_OpenCV".to_string(),
//     //         uid: "95e35702-d9e8-4f89-900f-7f4cfd5d40f6".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "WildlifeRecognizer".to_string(),
//     //         uid: "235750fe-6972-410a-8b68-0cf08c1eebb0".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "Recognizes animals".to_string(),
//     //         long_description: "Wildlife Recognition Application used to recognize wild animals".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/wlr_001.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "decisionTree".to_string(),
//     //         uid: "646cd6cb-23e1-428b-8275-8e8f2505939d".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "df_save".to_string(),
//     //         uid: "93889757-853a-49db-af7e-2fc9d1c49331".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "img_pose_recognizer".to_string(),
//     //         uid: "6e68cf95-571e-41ba-8f4c-63a240947492".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "linear".to_string(),
//     //         uid: "8f734e9b-f4a7-4291-8d22-40f4444adcf4".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "multiple object detection".to_string(),
//     //         uid: "6a25f4e0-2792-446e-ab45-2bf11ef41db2".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "pw_face_recogniser".to_string(),
//     //         uid: "7c6a1ab7-0824-4039-aab0-1859a4e9d9b9".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "pw_image_tokenizer".to_string(),
//     //         uid: "ad42a10f-b467-4f29-84f6-172260aed1a1".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "test 1".to_string(),
//     //         uid: "2fa5e3d3-86bc-4bd5-8fd9-4aa7341991be".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     },
//     //     BalticApp {
//     //         name: "tree visualizer".to_string(),
//     //         uid: "85063c65-6a4c-471a-b6de-c930da4a6356".to_string(),
//     //         p_class: "".to_string(),
//     //         short_description: "".to_string(),
//     //         long_description: "".to_string(),
//     //         icon: "https://www.balticlsc.eu/model/_icons/default.png".to_string(),
//     //         is_app: true,
//     //         is_service: false
//     //     }
//     // ];

//     html! {
//         <div>
//             <ul class="list">
//                 <AppsList apps={(*apps).clone()} />
//             </ul>
//         </div>
//     }
// }

// fn main() {
//     yew::Renderer::<App>::new().render();
// }

use crate::store::Store;
use yew::prelude::*;
use yew_router::prelude::*; // Assuming the Store component is implemented in store.rs

// mod router;
// mod store;

pub struct Router {}

impl Component for Router {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties) -> Self {
        Router {}
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Nav />
                <BrowserRouter>
                    <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
                </BrowserRouter>
            </div>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        "/" => html! { <Store /> },
        "/store" => html! { <Store /> },
        "/shelf" => html! { <Shelf /> },
        "/app/:id" => {
            let id = routes.param("id").unwrap_or_default();
            html! { <App id={id} /> }
        }
        _ => html! { <div>{"404 Not Found"}</div> },
    }
}

pub struct Nav {}

impl Component for Nav {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties) -> Self {
        Nav {}
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <nav>
                {"Test"}
            </nav>
        }
    }
}

pub struct Shelf {}

impl Component for Shelf {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties) -> Self {
        Shelf {}
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"Shelf"}</h1>
            </div>
        }
    }
}

pub struct App {
    id: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct AppProps {
    pub id: String,
}

impl Component for App {
    type Message = ();
    type Properties = AppProps;

    fn create(props: Self::Properties) -> Self {
        App { id: props.id }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"App"}</h1>
                <p>{format!("ID: {}", self.id)}</p>
            </div>
        }
    }
}

pub struct RouteContainerProps {
    children: Children,
}

struct RouteContainer {
    props: RouteContainerProps,
}

impl Component for RouteContainer {
    type Message = ();
    type Properties = RouteContainerProps;

    fn create(props: Self::Properties) -> Self {
        RouteContainer { props }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! { <>{ self.props.children.clone() }</> }
    }
}
