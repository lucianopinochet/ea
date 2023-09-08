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
pub type Records = (String,String, String, String, String, String, String, String, String, String, String, String, String,String, String);
#[derive(Clone, Copy)]
enum Sort{
  UP,
  DOWN,
  NONE
}
const FIELDS:[&str;15] = ["id","nombre", "patente", "fecha","hora","facturado","rut","kilometraje","motor_n","chassis_n","fono","informado","diagnostico","insumos_servicios","total"]; 
#[inline_props]
pub fn Inicio(cx: Scope) -> Element{
  // let toogle_query = use_state(cx, ||[false,false,false,false,false,false,false,false,false,false,false,false,false,false,false]);
  // let toogle_sort = use_state(cx, ||[false,false,false,false,false,false,false,false,false,false,false,false,false,false,false]);
  let query = use_state(cx, ||["","","","","","","","","","","","","","",""]);
  let sort = use_state(cx ,|| [Sort::NONE,Sort::NONE,Sort::NONE,Sort::NONE,Sort::NONE,Sort::NONE,Sort::NONE,Sort::NONE,Sort::NONE,Sort::NONE,Sort::NONE,Sort::NONE,Sort::NONE,Sort::NONE,Sort::NONE]);
  let file =  match File::open("data.csv"){
    Ok(file) => file,
    Err(_) => {
      let f = File::create("data.csv").unwrap();
      let mut wtr = csv::Writer::from_writer(f);
      wtr.write_record(&FIELDS).unwrap();
      File::open("data.csv").unwrap()
    }
  };
  let mut rdr = ReaderBuilder::new()
    .flexible(true)
    .comment(Some(b'#'))
    .from_reader(file);
  let node_list = rdr.deserialize();  
  let mut node_list:Vec<_> = node_list.map(|res|{
    let (id, nombre, patente, fecha, hora, facturado, rut, kilometraje, motor_n, chassis_n,fono, informado, diagnostico, insumos_servicios,total):Records = res.unwrap();
    (id, nombre, patente, fecha, hora, facturado, rut, kilometraje, motor_n, chassis_n, fono, informado, diagnostico, insumos_servicios, total)
  }).collect();
  for (index, field) in sort.get().iter().enumerate(){
    match field{
      Sort::UP => {
        node_list.sort_by(|a, b| {
          let a = [&a.0,&a.1,&a.2,&a.3,&a.4,&a.5,&a.6,&a.7,&a.8,&a.9,&a.10,&a.11,&a.12,&a.13,&a.14];
          let b = [&b.0,&b.1,&b.2,&b.3,&b.4,&b.5,&b.6,&b.7,&b.8,&b.9,&b.10,&b.11,&b.12,&b.13,&b.14];

          let a = a[index];
          let b = b[index];
          a.to_uppercase().partial_cmp(&b.to_uppercase()).unwrap()
        });
        break
      },
      Sort::DOWN => {
        node_list.sort_by(|a, b| {
          let a = [&a.0,&a.1,&a.2,&a.3,&a.4,&a.5,&a.6,&a.7,&a.8,&a.9,&a.10,&a.11,&a.12,&a.13,&a.14];
          let b = [&b.0,&b.1,&b.2,&b.3,&b.4,&b.5,&b.6,&b.7,&b.8,&b.9,&b.10,&b.11,&b.12,&b.13,&b.14];

          let a = a[index];
          let b = b[index];
          a.to_uppercase().partial_cmp(&b.to_uppercase()).unwrap()
        });
        node_list.reverse();
        break
      },
      Sort::NONE => {
        break
      },
    }
  }
  let rendered_body = node_list.iter().map(|result|{
    // let result = *result;
    let (id, nombre, patente, fecha, hora, facturado, rut, kilometraje, motor_n, chassis_n, fono, informado, diagnostico, insumos_servicios, total):Records = result.clone();
    render!{
      tr{td{"{id}"}td{"{nombre}"}td{"{patente}"}td{"{fecha}"}td{"{hora}"}td{"{facturado}"}td{"{rut}"}td{"{kilometraje}"}td{"{motor_n}"}td{"{chassis_n}"}td{"{fono}"}td{"{informado}"}td{"{diagnostico}"}td{"{insumos_servicios}"}td{"{total}"}
        // td{
        //   div{
        //     class:"icon-option",
        //     Link{
        //       to:Route::Record{
        //         id:id
        //       },
        //       Icon {
        //         width:15,
        //         height:15,
        //         icon: BsInfoCircleFill,
        //         class:"icon"
        //       }
        //     }
        //   }
        // }
      }
    }
  });
  let render_head:Vec<_> = FIELDS.iter().enumerate().map(|(index, field)|{
    render!{
      th{
        "{field}"
        div{
          onclick:move |_|{
            match sort.get()[index]{
              Sort::NONE => {
                let list = [Sort::NONE;15];
                list[index] = Sort::UP;
                sort.set(list);
              },
              Sort::UP => {
                let list = [Sort::NONE;15];
                list[index] = Sort::DOWN;
                sort.set(list);
              },
              Sort::DOWN => {
                let list = [Sort::NONE;15];
                sort.set(list);
              }
            }
          },
          Icon{
            width:15,
            height:15,
            class:"icon",
            icon:BsArrowDownCircleFill
          }
        }
      }
    }
  }).collect();
  let render_head = render_head.into_iter();
  render!{
    table{
      tr{
        render_head
        th{
          // style:"width:5%",
          h4{"Options"}
        }
      }
      rendered_body
    }
  }
}