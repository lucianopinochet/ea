use dioxus_router::prelude::*;
use dioxus::prelude::*;
use std::fs::File;
use csv::ReaderBuilder;
use dioxus_free_icons::icons::{
  fa_solid_icons::FaMagnifyingGlass,
  bs_icons::{BsInfoCircleFill, BsArrowDownCircleFill},
};
use crate::Route;
use std::collections::HashMap;
use dioxus_free_icons::Icon;
pub type Records = (u16,String, String, String, String, String, String, String, String, String, String, String, String, u32);
enum Sort{
  UP,
  DOWN,
  NONE
}
#[inline_props]
pub fn Inicio(cx: Scope) -> Element{
  let toogle_query = use_state(cx, ||[false,]);
  render!{
    table{
      tr{
        th{
          style:"width:10%",
          h4{"Id"}
        },
        render_nombre,
        render_patente,
        render_fecha,
        render_hora,
        render_facturado,
        render_rut,
        render_kilometraje,
        render_motor_n,
        render_chassis_n,
        render_fono,
        render_informado,
        render_diagnostico,
        render_insumos_servicios,
        render_total,
        th{
          style:"width:10%",
          h4{"Options"}
        }
      }
      rendered_body
    }
  }
}