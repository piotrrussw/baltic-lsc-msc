use yew::prelude::*;

pub struct App;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

impl Component for App {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"App"}</h1>
                <div>{ctx.props().id.clone()}</div>
            </div>
        }
    }
}
