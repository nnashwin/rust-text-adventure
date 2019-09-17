#![recursion_limit = "512"]

use serde_derive::{Deserialize, Serialize};
use yew::events::IKeyboardEvent;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
#[path = "engine/engine.rs"]
mod engine;

#[derive(Serialize, Deserialize)]
struct Entry {
    text: String,
    author: String,
}

pub struct Model {
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<Entry>,
    value: String,
}

pub enum Msg {
    Add,
    Update(String),
    None,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let state = State {
            entries: Vec::new(),
            value: "".into(),
        };

        Model { state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                let entry = Entry {
                    text: self.state.value.clone(),
                    author: "player".to_string(),
                };

                self.state.entries.push(entry);
                // TODO: Add logic to spit back output from the engine
                // self.state.value = engine::take_input(self.state.value.clone());
                self.state.value = "".to_string();
            }
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.state.value = val;
            }
            Msg::None => {}
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="webapp-wrapper">
                { self.view_input() }

                <div>
                    <div>{ "Entries" }</div>
                    <div>{ for self.state.entries.iter().enumerate().map(view_entry) }</div>
                </div>
            </div>
        }
    }
}

impl Model {
    fn view_input(&self) -> Html<Model> {
        html! {
            <input
                placeholder="What do you want to say?"
                value=&self.state.value
                oninput=|e| Msg::Update(e.value)
                onkeypress=|e| {
                   if e.key() == "Enter" { Msg::Add } else { Msg::None }
                } />
        }
    }
}

fn view_entry((idx, entry): (usize, &Entry)) -> Html<Model> {
    html! {
        <div>{ &entry.text }</div>
    }
}
