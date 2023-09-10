use csv::{Reader, Writer};
use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use std::fs::File;
use crate::Route;
use chrono::prelude::*;
use itertools::concat;
#[inline_props]
pub fn Agregar(cx: Scope) -> Element{
  let mut id:u16 = 0;
  let mut first = true;
  let date:DateTime<Local> = Local::now(); 
  let date = format!("{}-{}-{}",date.year(), date.month(), date.day());
  let time:NaiveTime = Local::now().time();
  let time = format!("{}:{}",time.hour(), time.minute());
  let input_values = use_state(cx, ||vec!["".to_string(),"".to_string(),date,time,"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string()]);
  let mut rdr = Reader::from_reader(File::open("data.csv").unwrap());
  let mut iter = rdr.records();
  while let Some(res) = iter.next(){
    first = false;
    let record = res.unwrap();
    id = record.get(0).unwrap().parse::<u16>().unwrap();
  }
  let file =  match File::options().append(true).open("data.csv"){
    Ok(file) => {
      file
    },
    Err(_) => {
      File::options().append(true).open("data.csv").unwrap()
    }
  };
  let mut wtr  = Writer::from_writer(file);
  let informe_input = use_state(cx,||"".to_string());
  let informado_list:&UseState<Vec<String>> = use_state(cx, ||vec![]);
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
  render!{
    form{
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
        class:"submit-input",
        Link{
          to:Route::Inicio{},
          onclick:move|_|{
            if !first{
              id+=1;
            }
            let inform = informado_list.get().clone();
            let inform = inform.join(";");
            let formated = format!("{id}").to_string();
            let list  = concat([vec![formated], input_values.get().clone(), vec![inform,"".to_string(),"".to_string(),"".to_string()]]);
            wtr.write_record(&list).unwrap();
            wtr.flush().unwrap();
          },
          input{
            class:"add-submit",
            r#type:"submit",
            value:"Enviar"
          }
        }
      }
    }
  }
}