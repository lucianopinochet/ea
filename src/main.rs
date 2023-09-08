#![allow(non_snake_case)]
// #![windows_subsystem = "windows"]
mod components;

use components::{NavBar, NotFound, Inicio};
use dioxus_router::prelude::*;
use dioxus::prelude::*;
use dioxus_desktop::tao::window::*;
fn main() {
  dioxus_desktop::launch_cfg(app, dioxus_desktop::Config::new()
    .with_window(WindowBuilder::new()
      .with_title("Check In")
      .with_resizable(true)
      .with_minimizable(true)
      .with_maximizable(true)
    ));
}

fn app(cx: Scope) -> Element {
  render!{
    style {include_str!("./style.css")},
    Router::<Route> {}
  }
}

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route{
  #[layout(NavBar)]
    #[route("/")]
    Inicio{},
  #[end_layout]
	#[route("/..notfound")]
	NotFound{}
}