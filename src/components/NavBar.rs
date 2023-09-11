use dioxus::prelude::*;
use crate::Route;
use dioxus_router::prelude::*;
use dioxus_free_icons::{
  icons::{
    // md_action_icons::MdHome,
    io_icons::{IoAdd, IoHome}
  },
  Icon
};
#[inline_props]
pub fn NavBar(cx: Scope) -> Element{
  render!{
    Outlet::<Route> {}
    div{
      class:"navbar",
      Link{
        to:Route::Inicio {},
        Icon{
          width:30,
          height:30,
          icon:IoHome,
          class:"icon"
        }
      }
      Link{
        to:Route::Agregar{},
        Icon{
          width:40,
          height:40,
          icon:IoAdd,
          class:"icon"
        }
      }  
    }
  }
}