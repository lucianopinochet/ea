use dioxus_router::prelude::*;
use dioxus::prelude::*;
use std::fs::File;
use csv::ReaderBuilder;
use dioxus_free_icons::icons::{
  fa_solid_icons::FaMagnifyingGlass,
  bs_icons::{BsInfoSquareFill, BsArrowDownCircleFill},
};
use crate::Route;
use dioxus_free_icons::Icon;
pub type Records = (String,String, String, String, String, String, String, String, String, String, String, String, String,String, String);
#[derive(Clone, Copy, Debug)]
enum Sort{
  UP,
  DOWN,
  NONE
}
const FIELDS:[&str;15] = ["Id","Nombre", "Patente", "Fecha","Hora","Facturado","Rut","Kilometraje","Motor N","Chassis N","Fono","Informado","Diagnostico","Insumos y Servicios","Total"]; 
#[inline_props]
pub fn Inicio(cx: Scope) -> Element{
  let toggle_query = use_state(cx ,||[false,false,false,false,false,false,false,false,false,false,false,false,false,false,false]);
  let query = use_state(cx, ||["".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string()]);
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
      Sort::NONE => (),
    }
  }
  let node_list:Vec<_> = node_list.iter().filter(|&row|{
    let list = [&row.0,&row.1,&row.2,&row.3,&row.4,&row.5,&row.6,&row.7,&row.8,&row.9,&row.10,&row.11,&row.12,&row.13,&row.14];
    let mut filtered = true;
    for (index, filter) in query.get().iter().enumerate(){
      if !filter.is_empty(){
        if !list[index].to_uppercase().contains(&filter.to_uppercase()) {filtered = false}
      }
    }
    filtered
  }).collect();
  let rendered_body = node_list.iter().map(|result|{
    let result = *result;
    let (id, nombre, patente, fecha, hora, facturado, rut, kilometraje, motor_n, chassis_n, fono, informado, diagnostico, insumos_servicios, total):Records = result.clone();
    let informado = informado.split(';').collect::<Vec<&str>>().join(",");
    let diagnostico = diagnostico.split(';').collect::<Vec<&str>>().join(",");
    let list_is:Vec<&str> = insumos_servicios.split(';').collect::<Vec<&str>>();
    let mut done_is:Vec<&str> = Vec::new();
    for item in list_is{
      let item = item.split('?').collect::<Vec<&str>>();
      done_is.push(item[0]);
    }
    let insumos_servicios = done_is.join(",");
    render!{
      tr{td{"{id}"}td{"{nombre}"}td{"{patente}"}td{"{fecha}"}td{"{hora}"}td{"{facturado}"}td{"{rut}"}td{"{kilometraje}"}td{"{motor_n}"}td{"{chassis_n}"}td{"{fono}"}td{"{informado}"}td{"{diagnostico}"}td{"{insumos_servicios}"}td{"{total}"}
        td{
          div{
            class:"icon-option",
            Link{
              to:Route::Record{
                id:id.parse::<u16>().unwrap()
              },
              Icon {
                width:15,
                height:15,
                icon: BsInfoSquareFill,
                class:"icon"
              }
            }
          }
        }
      }
    }
  });
  let render_head:Vec<_> = FIELDS.iter().enumerate().map(|(index, field)|{
    if !toggle_query.get()[index]{
      render!{
        th{
          h4{"{field}"}
          div{
            class:"table-head",
            div{
              onclick:move |_| {
                let mut toogle = toggle_query.get().clone();
                toogle[index] = true;
                toggle_query.set(toogle);
              },
              Icon {
                width:15,
                height:15,
                icon: FaMagnifyingGlass,
                class:"icon"
              },
            }
            div{
              onclick:move |_|{
                match sort.get()[index]{
                  Sort::NONE => {
                    let mut list = [Sort::NONE;15];
                    list[index] = Sort::UP;
                    sort.set(list);
                  },
                  Sort::UP => {
                    let mut list = [Sort::NONE;15];
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
      }
    }else{
      let query_value = &query.get()[index];
      render!{
        th{
          div{
            class:"table-head",
            h4{"{field}"}
            div{
              onclick:move |_|{
                match sort.get()[index]{
                  Sort::NONE => {
                    let mut list = [Sort::NONE;15];
                    list[index] = Sort::UP;
                    sort.set(list);
                  },
                  Sort::UP => {
                    let mut list = [Sort::NONE;15];
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
          form{
            prevent_default:"onsubmit",
            input{
              r#type:"text",
              value:"{query_value}",
              oninput:move |e| {
                let mut list = query.get().clone();
                list[index] = e.value.clone();
                query.set(list.clone())
              }
            }
          }
        }
      }
    }
  }).collect();
  let render_head = render_head.into_iter();
  render!{
    table{
      thead{
        tr{
          render_head
          th{
            h4{"Options"}
          }
        }
      }
      tbody{
        rendered_body
      }
    }
  }
}