use yew::functional::*;
use yew::prelude::*;

use crate::StateContext;

use crate::types::types::*;

#[function_component(DataShelfList)]
pub fn data_shelf_list() -> Html {
    let state_ctx = use_context::<StateContext>().unwrap();
    let state = state_ctx.clone();
    let data_shelf = state.data_shelf.clone();

    if state.data_shelf.is_empty() {
        html! {
            <div class="row">
                <div class="panel" />
            </div>
        }
    } else {
        html! {
          <div class="panel full-width">
          <div class="panel-head panel-left">
            <p class="panel-title">{"Data sets"}</p>
          </div>
          <div class="panel-body pt-0">
            <table>
              <thead>
                <tr>
                  <th>{"Name"}</th>
                  <th>{"Edit"}</th>
                  <th>{"Multiplicity"}</th>
                  <th>{"Data Type"}</th>
                  <th>{"Data Structure"}</th>
                  <th>{"Access Type"}</th>
                  <th>{"Access values"}</th>
                  <th>{"Path values"}</th>
                  <th>{"Delete"}</th>
                </tr>
              </thead>

              <tbody>
                {for data_shelf.into_iter().map(|item| render_data_shelf_item(item))}
              </tbody>
            </table>
          </div>
        </div>
        }
    }
}

fn render_data_shelf_item(data_shelf: DataShelfType) -> Html {
    html! {
        <tr key={data_shelf.uid.clone()}>
            <td>{&data_shelf.name}</td>
            <td>
              <button class="button-primary button-small button-round">{"Edit"}</button>
            </td>
            <td>
              <p>{if data_shelf.multiplicity > 0 { "Multiple" } else { "Single" }}</p>
            </td>
            <td>
                <p>{data_shelf.dataTypeName.unwrap_or_else(|| "-".to_string())}</p>
            </td>
            <td>
                <p>{data_shelf.dataStructureName.unwrap_or_else(|| "-".to_string())}</p>
            </td>
            <td>
                <p>{data_shelf.accessTypeName.unwrap_or_else(|| "-".to_string())}</p>
            </td>
            <td>
                <pre>
                    {pretty_json(&data_shelf.accessValues)}
                </pre>
            </td>
            <td>
                <pre>
                    {pretty_json(&data_shelf.values)}
                </pre>
            </td>
            <td>
              <button class="button-primary button-small button-round">{"Delete"}</button>
            </td>
        </tr>
    }
}

fn pretty_json(input: &str) -> Html {
    match serde_json::from_str::<serde_json::Value>(input) {
        Ok(parsed) => match serde_json::to_string_pretty(&parsed) {
            Ok(formatted_json) => {
                html! { formatted_json }
            }
            Err(_) => {
                html! { "-" }
            }
        },
        Err(_) => {
            html! { "-" }
        }
    }
}
