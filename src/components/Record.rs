use std::fs::File;
use dioxus::prelude::*;
use csv::{Reader, Writer, StringRecord};
use dioxus_router::prelude::Link;
use crate::components::Inicio::Records;
use crate::Route;
const FIELDS:[&str;15] = ["id","nombre", "patente", "fecha","hora","facturado","rut","kilometraje","motor_n","chassis_n","fono","informado","diagnostico","insumos_servicios","total"]; 
#[inline_props]
pub fn Record(cx:Scope, id:u16) -> Element{
  let input_values = use_state(cx, ||["".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string()]);
  let inputs =  FIELDS.iter().enumerate().map(|(index, field)|{
    render!{
      div{
        label{
          r#for:"{field}",
          "{field}"
        }
        input{
          r#type:"text",
          name:"{field}",
          value:"{input_values.get()[index]}",
          oninput:move|e|{
            let mut list = input_values.get().clone();
            list[index] = e.value.clone();
            input_values.set(list.clone())
          }
        }
      }
    }
  });
  // let delete_button = 
  render!{
    form{
      class:"check-in-form",
      prevent_default:"onsubmit",
      inputs
      div{
        Link{
          to:Route::Inicio{},
          onclick:move|_|{

          },
          input{
            r#type:"submit",
            value:"Enviar"
          }
        }
      }
      // delete_button
    }
  }
}