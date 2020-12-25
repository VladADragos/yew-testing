 
use wasm_bindgen::JsCast;
use yew::web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use yew::services::render::RenderTask;
use yew::services::RenderService;
use yew::{html, Component, ComponentLink,NodeRef,ShouldRender};

use super::renderer::{Renderer,Rect,Origin};

pub enum Msg{
    Render(f64),
    Add

}

pub struct Canvas{
    canvas: Option<HtmlCanvasElement>,
    renderer: Option<Renderer>,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    render_loop: Option<RenderTask>,
    prev_timestamp: f64,
    rect: Rect,
    rects: [Rect;8]

}

impl Component for Canvas{
    type Message = Msg;    
    type Properties = ();

    fn create(_props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            canvas: None,
            renderer: None,
            link,
            node_ref: NodeRef::default(),
            render_loop: None,
            prev_timestamp: 0.0,
            rect: Rect{origin:Origin{x:0,y:0},width:100,height:50},
            rects: [Rect::default();8]

        }
    }
    fn rendered(&mut self, first_render: bool) {
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

        let ctx:CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
        self.canvas = Some(canvas);
        self.renderer = Some(Renderer::new(ctx,200,200));

        if first_render {
            let mut diff = 0;
            let mut i = 0;
            for rect in &mut self.rects{
                rect.height = 10;
                rect.width = 10;
                rect.origin.x = diff+(10*i);
                diff += 10;
                i += 1;
            }
            // The callback to request animation frame is passed a time value which can be used for
            // rendering motion independent of the framerate which may vary.
            let render_frame = self.link.callback(Msg::Render);
            let handle = RenderService::request_animation_frame(render_frame);

            // A reference to the handle must be stored, otherwise it is dropped and the render won't
            // occur.
            self.render_loop = Some(handle);
        }

    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // log::info!("Update: {:?}","TETS");
        match msg {
            Msg::Render(timestamp) => {
                // Render functions are likely to get quite large, so it is good practice to split
                // it into it's own function rather than keeping it inline in the update match
                // case. This also allows for updating other UI elements that may be rendered in
                // the DOM like a framerate counter, or other overlaid textual elements.
                // self.render_gl(timestamp);
                // log("hello world");
                // if let Some(prev_timestamp)= self.prev_timestamp{
                //     let new_timestamp= prev_timestamp-timestamp;
                //     self.prev_timestamp = Some(new_timestamp);
                // }else{
                //     self.prev_timestamp = Some(timestamp);
                // }
              
                
                self.rect.origin.x += 10;

                
                
                if let Some(_renderer) = &self.renderer{
                    _renderer.clear_all();
                    _renderer.draw_rects(&self.rects);
                } else{
                    panic!("renderer error");
                }
                for rect in &mut self.rects{
                    rect.origin.y += 15;
                    rect.origin.x += 15;
                }
                let render_frame = self.link.callback(Msg::Render);
                let handle = RenderService::request_animation_frame(render_frame);
        
                // A reference to the new handle must be retained for the next render to run.
                self.render_loop = Some(handle);
                false
            },
            Msg::Add => {
                log::info!("adding: {:?}","TETS");
                self.rect.origin.x += 10;
                true
            }
        }

    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        html! {
            <div> 
            
            <canvas ref=self.node_ref.clone() />
            <button onclick=self.link.callback(|_| Msg::Add) >{"up"}</button>
            {self.rect.origin.x}
            </div>
        }
    }


}
