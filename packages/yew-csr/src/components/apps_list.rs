use chrono::DateTime;
use chrono::Utc;

use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::StateContext;

use crate::types::types::*;

fn date_sort(a: &AppType, b: &AppType) -> std::cmp::Ordering {
    let date_a = a
        .releases
        .last()
        .and_then(|release| {
            let date_with_z = format!("{}Z", release.date);
            DateTime::parse_from_rfc3339(&date_with_z).ok()
        })
        .map(|datetime| datetime.timestamp())
        .unwrap_or(i64::MAX);

    let date_b = b
        .releases
        .last()
        .and_then(|release| {
            let date_with_z = format!("{}Z", release.date);
            DateTime::parse_from_rfc3339(&date_with_z).ok()
        })
        .map(|datetime| datetime.timestamp())
        .unwrap_or(i64::MAX);

    date_a.cmp(&date_b)
}

fn name_sort(a: &AppType, b: &AppType) -> std::cmp::Ordering {
    a.name.cmp(&b.name)
}

fn get_filtered_data(state: StateContext) -> Vec<AppType> {
    let search_value = state.search.to_lowercase();
    let apps_cloned = state.apps.clone();
    let shelf_cloned = state.shelf.clone();

    let mut data: Vec<AppType> = apps_cloned
        .into_iter()
        .map(|app| {
            let in_cockpit: Option<bool> = if shelf_cloned.iter().any(|s| s.unit.uid == app.uid) {
                Some(true)
            } else {
                None
            };

            AppType {
                inCockpit: in_cockpit,
                ..app
            }
        })
        .filter(|d| {
            d.name.to_lowercase().contains(&search_value)
                || d.shortDescription
                    .as_ref()
                    .map(|s| s.to_lowercase())
                    .unwrap_or_default()
                    .contains(&search_value)
        })
        .collect();

    if state.sort == "date" {
        data.sort_by(|a, b| date_sort(a, b));
    } else {
        data.sort_by(|a, b| name_sort(a, b));
    }

    data
}

#[function_component(AppsList)]
pub fn apps_list() -> Html {
    let state_ctx = use_context::<StateContext>().unwrap();
    let state = state_ctx.clone();
    let filtered_data = get_filtered_data(state_ctx);

    if state.apps.is_empty() || state.shelf.is_empty() {
        html! {
            <div class="row">
                <div class="col col-md-4">
                <div class="card card-max-height skeleton" />
                </div>
                <div class="col col-md-4">
                <div class="card card-max-height skeleton" />
                </div>
                <div class="col col-md-4">
                <div class="card card-max-height skeleton" />
                </div>
            </div>
        }
    } else {
        html! {
            <div class="row">
            {
                filtered_data.iter()
                    .map(|it| {
                        let date_str = it.releases.last().map(|release| &release.date);
                        let last_update = date_str
                        .map(|s| s.to_owned() + "Z") // to_owned() to convert &str to String
                        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                        .map(|datetime| datetime.with_timezone(&Utc));

                        let button_class = if it.inCockpit.unwrap_or(false)  {
                            "button-primary-outlined button-small button-round"
                        } else {
                            "button-primary button-small button-round"
                        };


                        html! {
                            <div key={it.uid.clone()} class="col col-md-4">
                                <div class="card card-max-height">
                                    <div class="row inline-card-title">
                                        <img src={it.icon.clone()} width={50} height="auto" />
                                        <div class="card-title">
                                            <Link<Route> to={Route::AppRoute { id: it.uid.to_string() }}>{format!("{}", it.name)}</Link<Route>>
                                        </div>
                                    </div>
                                    <p>
                                    { match &it.shortDescription {
                                        Some(desc) => format!("{}", desc),
                                        None => "".to_string(),
                                    }}
                                    </p>
                                    <div>
                                        <strong>{"Updated on: "}</strong>
                                        { match last_update {
                                            Some(date) => format!("{}", date.format("%Y-%m-%d %H:%M:%S")),
                                            None => "-".to_string(),
                                        }}
                                    </div>
                                    <div class="card-actions">
                                    <button class={button_class}>
                                        // TODO: add onClick
                                        {if it.inCockpit.unwrap_or(false) {"In cockpit"} else {"Add to cockpit"}}
                                    </button>
                                  </div>
                                </div>
                            </div>

                        }
                    })
                    .collect::<Html>()
            }
            </div>
        }
    }
}
