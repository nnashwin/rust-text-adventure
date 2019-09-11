#![recursion_limit = "512"]
mod lib;

fn main() {
    yew::start_app::<lib::Model>();
}
