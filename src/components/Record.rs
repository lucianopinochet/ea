use dioxus::prelude::*;
use csv::{Reader, Writer, StringRecord};
use dioxus_router::prelude::Link;
use crate::components::Inicio::Records;
use crate::Route;
use itertools::concat;
#[inline_props]
pub fn Record(cx:Scope, id:u16) -> Element{
  let delete = use_state(cx, ||false);
  let mut rdr = Reader::from_path("data.csv").unwrap();
  let records = rdr.deserialize();
  let mut record = ("".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string());
  for res in records{
    let (rec_id,nombre,patente,fecha,hora,facturado,rut,kilometraje,motor_n,chassis_n,fono,informado,diagnostico,insumos_servicios,_):Records = res.unwrap();
    if rec_id == id.to_string(){
      record = (nombre,patente,fecha,hora,facturado,rut,kilometraje,motor_n,chassis_n,fono,informado,diagnostico,insumos_servicios);
      break
    }
  }
  let input_values = use_state(cx, ||vec![record.0,record.1,record.2,record.3,record.4,record.5,record.6,record.7,record.8,record.9]);
  let informe_input = use_state(cx,||"".to_string());
  let mut rdr_informe = vec![];
  if !record.10.is_empty(){
    rdr_informe = record.10.split(';').collect::<Vec<&str>>().iter().map(|value|value.to_string()).collect::<Vec<String>>();
  }
  let informado_list:&UseState<Vec<String>> = use_state(cx, ||rdr_informe);
  let informe = informado_list.get().iter().enumerate().map(|(index, item)|{
    render!{
        input{
          value:"{item}",
          style:"width:95%;",
          oninput:move|e| {
            let mut list = informado_list.get().clone();
            list[index] = e.value.clone();
            informado_list.set(list);
          }
        }
    }
  });
  let diagnostico_input = use_state(cx,||"".to_string());
  let mut rdr_diagnostico = vec![];
  if !record.11.is_empty(){
    rdr_diagnostico = record.11.split(';').collect::<Vec<&str>>().iter().map(|value|value.to_string()).collect::<Vec<String>>();
  }
  let diagnostico_list:&UseState<Vec<String>> = use_state(cx, ||rdr_diagnostico);
  let diagnostico = diagnostico_list.get().iter().enumerate().map(|(index, item)|{
    render!{
        input{
          value:"{item}",
          style:"width:95%;",
          oninput:move|e| {
            let mut list = diagnostico_list.get().clone();
            list[index] = e.value.clone();
            diagnostico_list.set(list);
          }
        }
    }
  });
  let is_input = use_state(cx,||"".to_string());
  let valor_input = use_state(cx,||"".to_string());
  let mut rdr_is = vec![];
  if !record.12.is_empty(){
    rdr_is = record.12.split(';').collect::<Vec<&str>>().iter().map(|value|{
      let splited = value.split('?').collect::<Vec<&str>>();
      (splited[0].to_string(),splited[1].to_string())
    }).collect::<Vec<(String, String)>>();
  }
  let is_list:&UseState<Vec<(String,String)>> = use_state(cx, ||rdr_is);
  let is = is_list.get().iter().enumerate().map(|(index, (item, value))|{
    render!{
        input{
          value:"{item}",
          style:"width:75%;",
          oninput:move|e| {
            let mut list = is_list.get().clone();
            list[index] = (e.value.clone(),list[index].1.clone());
            is_list.set(list);
          }
        }
        input{
          value:"{value}",
          r#type:"number",
          style:"width:15%;",
          oninput:move|e| {
            let mut list = is_list.get().clone();
            list[index] = (list[index].0.clone(),e.value.clone());
            is_list.set(list);
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
      div{
        class:"check-in-form",
        div{
          class:"add-input",
          label{
            r#for:"nombre",
            "Nombre"
          }
          input{
            r#type:"text",
            required:true,
            name:"nombre",
            value:"{input_values.get()[0]}",
            oninput:move|e|{
              let mut list = input_values.get().clone();
              list[0] = e.value.clone();
              input_values.set(list.clone())
            }
          }
        }
        div{
          class:"add-input",
          label{
            r#for:"patente",
            "Patente"
          }
          input{
            r#type:"text",
            required:true,
            name:"patente",
            pattern:"[a-zA-Z0-9]{6}",
            value:"{input_values.get()[1]}",
            oninput:move|e|{
              let mut list = input_values.get().clone();
              list[1] = e.value.clone();
              input_values.set(list.clone())
            }
          }
        }
        div{
          class:"add-input",
          label{
            r#for:"fecha",
            "Fecha"
          }
          input{
            r#type:"date",
            required:true,
            name:"fecha",
            value:"{input_values.get()[2]}",
            oninput:move|e|{
              let mut list = input_values.get().clone();
              list[2] = e.value.clone();
              input_values.set(list.clone())
            }
          }
        }
        div{
          class:"add-input",
          label{
            r#for:"hora",
            "Hora"
          }
          input{
            r#type:"time",
            required:true,
            name:"hora",
            value:"{input_values.get()[3]}",
            oninput:move|e|{
              let mut list = input_values.get().clone();
              list[3] = e.value.clone();
              input_values.set(list.clone())
            }
          }
        }
        div{
          class:"add-input",
          label{
            r#for:"facturado",
            "Facturado"
          }
          input{
            r#type:"text",
            required:true,
            name:"facturado",
            value:"{input_values.get()[4]}",
            oninput:move|e|{
              let mut list = input_values.get().clone();
              list[4] = e.value.clone();
              input_values.set(list.clone())
            }
          }
        }
        div{
          class:"add-input",
          label{
            r#for:"rut",
            "Rut"
          }
          input{
            r#type:"text",
            required:true,
            name:"rut",
            value:"{input_values.get()[5]}",
            oninput:move|e|{
              let mut list = input_values.get().clone();
              list[5] = e.value.clone();
              input_values.set(list.clone())
            }
          }
        }
        div{
          class:"add-input",
          label{
            r#for:"kilometraje",
            "Kilometraje"
          }
          input{
            r#type:"number",
            required:true,
            name:"kilometraje",
            value:"{input_values.get()[6]}",
            oninput:move|e|{
              let mut list = input_values.get().clone();
              list[6] = e.value.clone();
              input_values.set(list.clone())
            }
          }
        }
        div{
          class:"add-input",
          label{
            r#for:"motor_n",
            "Motor N"
          }
          input{
            r#type:"text",
            required:true,
            name:"motor_n",
            value:"{input_values.get()[7]}",
            oninput:move|e|{
              let mut list = input_values.get().clone();
              list[7] = e.value.clone();
              input_values.set(list.clone())
            }
          }
        }
        div{
          class:"add-input",
          label{
            r#for:"chassis_n",
            "Chassis N"
          }
          input{
            r#type:"text",
            required:true,
            name:"chassis_n",
            value:"{input_values.get()[8]}",
            oninput:move|e|{
              let mut list = input_values.get().clone();
              list[8] = e.value.clone();
              input_values.set(list.clone())
            }
          }
        }
        div{
          class:"add-input",
          label{
            r#for:"fono",
            "Fono"
          }
          input{
            r#type:"tel",
            required:true,
            name:"fono",
            placeholder:"+56912345678",
            pattern:"+569[0-9]{8}",
            value:"{input_values.get()[9]}",
            oninput:move|e|{
              let mut list = input_values.get().clone();
              list[9] = e.value.clone();
              input_values.set(list.clone())
            }
          }
        }
      }
      div{
        class:"informado",
        h4{"Informado"}
        form{
          prevent_default:"onsubmit",
          class:"informado-form",
          input{
            oninput:|e| informe_input.set(e.value.clone()),
            style:"width:90%;",
            r#type:"text",
            value:"{informe_input}"
          }
          input{
            onclick:move|_|{
              let mut list = informado_list.get().clone();
              list.push(informe_input.get().clone());
              informe_input.set("".to_string());
              informado_list.set(list.clone());
            },        
            class:"informado-button",
            style:"width:10%;",
            r#type:"button",
            value:"Agregar"
          }
        }
        informe
      }
      div{
        class:"informado",
        h4{"Diagnostico"}
        form{
          prevent_default:"onsubmit",
          class:"informado-form",
          input{
            oninput:|e| diagnostico_input.set(e.value.clone()),
            style:"width:90%;",
            r#type:"text",
            value:"{diagnostico_input}"
          }
          input{
            onclick:move|_|{
              let mut list = diagnostico_list.get().clone();
              list.push(diagnostico_input.get().clone());
              diagnostico_input.set("".to_string());
              diagnostico_list.set(list.clone());
            },        
            class:"informado-button",
            style:"width:10%;",
            r#type:"button",
            value:"Agregar"
          }
        }
        diagnostico
      }
      div{
        class:"informado",
        h4{"Insumos o Servicios"}
        form{
          prevent_default:"onsubmit",
          class:"informado-form",
          input{
            oninput:|e| is_input.set(e.value.clone()),
            style:"width:75%;",
            r#type:"text",
            value:"{is_input}"
          }
          input{
            oninput:|e| valor_input.set(e.value.clone()),
            style:"width:15%;",
            r#type:"number",
            value:"{valor_input}"
          }
          input{
            onclick:move|_|{
              let mut list = is_list.get().clone();
              list.push((is_input.get().clone(),valor_input.get().clone()));
              is_input.set("".to_string());
              valor_input.set("".to_string());
              is_list.set(list.clone());
            },        
            class:"informado-button",
            style:"width:10%;",
            r#type:"button",
            value:"Agregar"
          }
        }
        is
      }
      div{
        Link{
          to:Route::Inicio{},
          onclick:move|_|{
            let mut rdr = Reader::from_path("data.csv").unwrap();
            let headers = rdr.headers().unwrap().clone();
            let mut records:Vec<StringRecord> = Vec::new();
            let inform = informado_list.get().clone();
            let inform = inform.join(";");
            let diagnostic = diagnostico_list.get().clone();
            let diagnostic = diagnostic.join(";");
            let is = is_list.get().clone();
            let mut total:i32 = 0;
            let is:Vec<String> =  is.iter().map(|(item, value)|{
              total+=value.parse::<i32>().unwrap();
              format!("{item}?{value}")
            }).collect();
            let total = total.to_string();
            let is = is.join(";");
            for result in rdr.records(){
              let record = result.unwrap();
              if record.get(0).unwrap() == id.to_string(){
                records.push(StringRecord::from(concat([vec![id.to_string()],Vec::from(input_values.get().clone()),vec![inform.clone(),diagnostic.clone(),is.clone(),total.clone()]])));
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