use yew::prelude::*;

pub struct Shelf;

impl Component for Shelf {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"Shelf"}</h1>
            </div>
        }
    }
}
