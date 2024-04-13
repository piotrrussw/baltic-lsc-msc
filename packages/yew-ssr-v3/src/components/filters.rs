use web_sys::*;

use yew::functional::*;
use yew::prelude::*;

use crate::Action;
use crate::StateContext;

#[function_component(Filters)]
pub fn filters() -> Html {
    let state_ctx = use_context::<StateContext>().unwrap();
    let initial_sort = state_ctx.sort.clone(); // Get the initial value

    let on_input = {
        let state_ctx_clone = state_ctx.clone(); // Clone state_ctx

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            state_ctx_clone.dispatch(Action::UpdateSearch(input.value()))
        })
    };

    let on_select = {
        let state_ctx_clone = state_ctx.clone(); // Clone state_ctx

        Callback::from(move |event: Event| {
            let select = event.target_dyn_into::<HtmlSelectElement>().unwrap();
            state_ctx_clone.dispatch(Action::UpdateSort(select.value()))
        })
    };

    html! {
      <div class="row py-10">
        <div class="col col-sm-6">
          <div class="form-control">
          <input
            oninput={on_input}
            type="search"
            placeholder="Search app by name or description"
          />
          </div>
        </div>


        <div class="col col-sm-2">
          <div class="form-control">
            <select onchange={on_select} value={initial_sort}>
              <option value="name">{"Name"}</option>
              <option value="date">{"Date"}</option>
            </select>
          </div>
        </div>
    </div>
    }
}
