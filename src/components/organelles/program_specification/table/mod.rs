use stylist::{Style, style};
use yew::{Html, html, Properties, Children, Component, Context, MouseEvent};

#[derive(PartialEq,Properties)]
pub struct Props {}

pub enum Msg {}

pub struct TableBlock;

impl Component for TableBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self { 
        Self
    }

    // HTML stuff
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{"I am a table block!"}</p>
            </div>
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
}