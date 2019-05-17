// This is need for "js!" macros
#[macro_use]
extern crate stdweb;

use yew::services::{ConsoleService, Task};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

// Load our keydown service
mod keydown_service;
use keydown_service::KeydownService;

// Handle browser keyboard events and decode structures
use stdweb::traits::IKeyboardEvent;
use stdweb::web::event::KeyDownEvent;

// Standard YEW boilerplate
pub struct Model {
    keydown_service: KeydownService,
    keydown_cb: Callback<KeyDownEvent>,
    keydown_job: Option<Box<Task>>,

    console: ConsoleService,
    messages: Vec<String>,
}

pub enum Msg {
    ListenKeydown,
    HandleKeyDown(KeyDownEvent),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        Model {
            keydown_service: KeydownService::new(),
            keydown_cb: link.send_back(|e| Msg::HandleKeyDown(e)),
            keydown_job: None,

            console: ConsoleService::new(),
            messages: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ListenKeydown => {
                let handle = self.keydown_service.spawn(self.keydown_cb.clone());
                self.keydown_job = Some(Box::new(handle));
            }

            Msg::HandleKeyDown(e) => {
                self.messages.push(e.key());
                self.console.log(&e.key());
            }
        }

        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let view_message = |message| {
            html! { { message } }
        };

        html! {
            <div >
                <strong>{"YEW global keydown example."}</strong>
                <p>{"Click and Type:"}</p>
                <div>
                    { for self.messages.iter().map(view_message) }
                </div>
            </div>
        }
    }
}
