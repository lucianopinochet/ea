use dioxus::prelude::*;
use crate::Route;
use dioxus_router::prelude::*;

#[inline_props]
pub fn NavBar(cx: Scope) -> Element{
  render!{
    div{
      class:"navbar",
      Link{
        to:Route::Inicio {},
        "Inicio"
      }
      Link{
        to:Route::Inicio {},
        "Agregar"
      }  
    }
    Outlet::<Route> {}
  }
}