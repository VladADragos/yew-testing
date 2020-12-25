 
// use wasm_bindgen::JsCast;
// use yew::web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
// use yew::services::render::RenderTask;
// use yew::services::RenderService;
use yew::{html, Component};

use crate::components::canvas::canvas::Canvas;

// use super::components::nav;
pub enum Msg{
    Render(f64)
}

pub struct MyCanvas;

impl Component for MyCanvas{
    type Message = Msg;    
    type Properties = ();

    fn create(_props: Self::Properties, _link: yew::ComponentLink<Self>) -> Self {
        Self {}
    }

    
    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        true
    }
    fn update(&mut self, _msg: Self::Message) -> yew::ShouldRender {
        true
    }

    fn view(&self) -> yew::Html {
        html! {
            <Canvas/>
        }
    }


}

