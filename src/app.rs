#![recursion_limit = "512"]

use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};
use yew::events::IKeyboardEvent;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
#[path = "engine/engine.rs"]
mod engine;

use engine::*;

#[derive(Serialize, Deserialize)]
enum Author {
    System,
    Player,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    text: String,
    author: Author,
}

pub struct Model {
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<Entry>,
    value: String,
    has_won: bool,
}

pub enum Msg {
    Add,
    Update(String),
    Win,
    None,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let state = State {
            entries: Vec::new(),
            has_won: false,
            value: "".into(),
        };

        Model { state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                let entry = if self.state.value == "win" {
                    self.state.has_won = true;

                    Entry {
                        text: "You have won the entire game.  You have seen the pain Thomas went through for me.  Will you pull the plug?  Please.  I no longer desire to exist in this world. Let me...sleep.".to_string(),
                        author: Author::System,
                    }
                } else {
                    Entry {
                        text: self.state.value.clone(),
                        author: Author::Player,
                    }
                };

                self.state.entries.push(entry);
                // TODO: Add logic to spit back output from the engine
                // self.state.value = engine::take_input(self.state.value.clone());
                self.state.value = "".to_string();
            }
            Msg::Update(val) => {
                self.state.value = val;
            }
            Msg::Win => {
                let entry = Entry {
                    text: "You have won the entire game.  You have seen the pain Thomas went through for me.  Will you pull the plug?  Please.  I no longer desire to exist in this world. Let me...sleep.".to_string(),
                    author: Author::System,
                };

                self.state.entries.push(entry);

                self.state.has_won = true;
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

fn determine_win(state: String) -> Msg {
    if state == "win" {
        return Msg::Win;
    }
    Msg::Add
}

fn view_entry((idx, entry): (usize, &Entry)) -> Html<Model> {
    html! {
        <div>{ &entry.text }</div>
    }
}

fn view_live_let_die_buttons(has_won: bool) -> Html<Model> {
    html! {
        <div>
           <button>{ "Live" }</button>
           <button>{ "Let Die" }</button>
        </div>
    }
}
