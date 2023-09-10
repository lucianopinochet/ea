use dioxus::prelude::*;
use csv::{Reader, Writer, StringRecord};
use dioxus_router::prelude::Link;
use crate::components::Inicio::Records;
use crate::Route;
const FIELDS:[&str;15] = ["Id","Nombre", "Patente", "Fecha","Hora","Facturado","Rut","Kilometraje","Motor N","Chassis N","Fono","Informado","Diagnostico","Insumos y Servicios","Total"]; 
#[inline_props]
pub fn Record(cx:Scope, id:u16) -> Element{
  let delete = use_state(cx, ||false);
  let mut rdr = Reader::from_path("data.csv").unwrap();
  let records = rdr.deserialize();
  let mut record:Records = ("".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string());
  for res in records{
    let (rec_id,nombre,patente,fecha,hora,facturado,rut,kilometraje,motor_n,chassis_n,fono,informado,diagnostico,insumos_servicios,total):Records = res.unwrap();
    if rec_id == id.to_string(){
      record = (rec_id,nombre,patente,fecha,hora,facturado,rut,kilometraje,motor_n,chassis_n,fono,informado,diagnostico,insumos_servicios,total);
      break
    }
  }
  let input_values = use_state(cx, ||[record.0,record.1,record.2,record.3,record.4,record.5,record.6,record.7,record.8,record.9,record.10,record.11,record.12,record.13,record.14]);
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
  let delete_button = if delete.get().clone() == false{
    render!{
      div{
        input{
          class:"add-submit",
          r#type:"button",
          value:"Eliminar",
          prevent_default:"onsubmit",
          onclick:move|_|delete.set(true)
        }
      }
    }
  }else{
    render!{
      div{
        Link{
          to:Route::Inicio{},
          onclick:move|_|{
            let mut rdr = Reader::from_path("data.csv").unwrap();
            let headers = rdr.headers().unwrap().clone();
            let mut records:Vec<StringRecord> = Vec::new();
            for result in rdr.records(){
              let record = result.unwrap();
              if record.get(0).unwrap() != id.to_string(){
                records.push(record);
              }
            }
            let mut wrt = Writer::from_path("data.csv").unwrap();
            wrt.write_record(&headers).unwrap();
            for record in records{
              wrt.write_record(&record).unwrap();
            }
            wrt.flush().unwrap();
          },
          input{
            class:"add-submit",
            r#type:"submit",
            value:"Seguro?"
          }
        }
      }
    }
  };
  render!{
    form{
      class:"check-in2-form",
      prevent_default:"onsubmit",
      inputs
      div{
        Link{
          to:Route::Inicio{},
          onclick:move|_|{
            let mut rdr = Reader::from_path("data.csv").unwrap();
            let headers = rdr.headers().unwrap().clone();
            let mut records:Vec<StringRecord> = Vec::new();
            for result in rdr.records(){
              let record = result.unwrap();
              if record.get(0).unwrap() == id.to_string(){
                records.push(StringRecord::from(Vec::from(input_values.get().clone())));
              }else{
                records.push(record);
              }
            }
            let mut wrt = Writer::from_path("data.csv").unwrap();
            wrt.write_record(&headers).unwrap();
            for record in records{
              wrt.write_record(&record).unwrap();
            }
            wrt.flush().unwrap();
          },
          input{
            class:"add-submit",
            r#type:"submit",
            value:"Editar"
          }
        }
      }
      delete_button
    }
  }
}