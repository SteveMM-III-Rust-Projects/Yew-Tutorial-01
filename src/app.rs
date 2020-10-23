//use rand::prelude::*;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::format::Json;
use yew::services::ConsoleService as console;
use yew::services::DialogService as dialog;
use yew::services::storage::Area;
use yew::services::StorageService;
use serde::{ Deserialize, Serialize };


const KEY: &'static str = "yew.tut.database";


#[derive(Serialize, Deserialize)]
pub struct Database {
   tasks: Vec<Task>,
}


impl Database {
   pub fn new() -> Self {
      return Database {
         tasks: Vec::new()
      }
   }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
   title: String,
   description: String,
}


impl Task {
   pub fn new() -> Self {
      return Task {
         title: "".to_string(),
         description: "".to_string(),
      };
   }


   pub fn is_filled_in( &self ) -> bool {
      return ( self.title != "" ) && ( self.description != "" );
   }
}


pub enum Msg {
   AddTask,
   RemoveTask,
   About,
}


pub struct App {
   items: Vec<i64>,
   counter: i64,
   link: ComponentLink<Self>,
}


impl Component for App {
   type Message = Msg;
   type Properties = ();


   fn create( _: Self::Properties, link: ComponentLink<Self> ) -> Self {
      return App {
         link,
         items: Vec::new(),
         counter: 0,
      };
   }


   fn update( &mut self, msg: Self::Message ) -> ShouldRender {
      match msg {
         Msg::AddTask => {
            let added = random();
            self.counter += 1;
            self.items.push( added );
            console::info( format!( "Added {}", added ).as_str() );
         }
         Msg::RemoveTask => {
            self.counter -= if self.counter == 0 { 0 } else { 1 };
            let removed = self.items.pop();
            match removed {
               Some(x) => console::warn( format!( "Removed {}", x ).as_str() ),
               None    => console::error( "Nothing to remove!" )
            }
         }
         Msg::About => {
            dialog::alert( "training app" )
         }
      }

      return true;
   }


   fn view( &self ) -> Html {
      let render_item = |item| {
         html! {
            <li> {item} </li>
         }
      };

      return html! {
         <div class="app center">
            <p> {"Number of items: "} { self.counter } </p>
            <p> {"Items: "}</p>

            <ul>
               { for self.items.iter().map( render_item ) }
            </ul>

            <div class="add-remove">
               <button onclick=self.link.callback( |_| Msg::AddTask    )>
                  { "+" }
               </button>
               <button onclick=self.link.callback( |_| Msg::RemoveTask )>
                  { "-" }
               </button>
            </div>

            <button class="about" onclick=self.link.callback(|_| Msg::About )>
               { "About" }
            </button>
         </div>
      };
   }


   fn change( &mut self, _props: Self::Properties ) -> ShouldRender {
      return true;
   }
}
