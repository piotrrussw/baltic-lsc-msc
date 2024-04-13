use yew::prelude::*;

use crate::StateContext;

#[function_component(AppView)]
pub fn app_view() -> Html {
    let state_ctx = use_context::<StateContext>().unwrap();
    let state = state_ctx.clone();
    let app = state.app.clone();

    html! {
        <div class="v-stack">
            <div class="panel full-width">
                <div class="panel-head panel-left">
                  <img src={app.icon} width="50" height="auto" />
                  <p class="panel-title">{&app.name}</p>
                </div>
                <div class="panel-body">
                  <p>{if let Some(long_description) = &app.longDescription { long_description } else { "No description" }}</p>
                </div>
                <div class="panel-footer">
                  <ul class="tags">
                  {match &app.isApp {
                      Some(true) => html! { <li class="tag tag-green">{"App"}</li> },
                      _ => html! {}
                  }}

                  {match &app.isService {
                      Some(true) => html! { <li class="tag tag-blue">{"Service"}</li> },
                      _ => html! {}
                  }}
                  </ul>
                </div>
            </div>

            <div class="panel full-width">
              <div class="panel-head panel-left">
                <p class="panel-title">{"Releases"}</p>
              </div>
              <div class="panel-body pt-0">
                <table>
                  <thead>
                    <tr>
                      <th>{"Version name"}</th>
                      <th>{"Release date"}</th>
                      <th>{"Description"}</th>
                      <th>{"OpenSource"}</th>
                      <th>{"Status"}</th>
                    </tr>
                  </thead>
                  <tbody>
                    { for app.releases.iter().map(|release| html! {
                        <tr key={release.uid.clone()}>
                            <td>
                                <p>
                                    <strong>{&release.version}</strong>
                                </p>
                            </td>
                            <td>
                                <p>
                                {&release.date}
                            // TODO: check app_list
                            //       {format!("{}", release.date.format("%Y-%m-%d %H:%M:%S"))}
                                </p>
                            </td>
                            <td>
                                <p>{release.description.as_ref().map(String::as_str).unwrap_or("-")}</p>
                            </td>
                            <td>
                                <p>{match release.openSource {
                                    Some(true) => "Yes",
                                    Some(false) => "No",
                                    _ => "N/A",
                                }}</p>
                            </td>
                            <td>
                                <p>{if release.status == 0 { "Created" } else { "Approved" }}</p>
                            </td>
                        </tr>
                    })}
                  </tbody>
                </table>
              </div>
            </div>
        </div>
    }
}
