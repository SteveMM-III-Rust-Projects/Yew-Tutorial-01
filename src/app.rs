use super::js_funcs::refreshform;

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
   RemoveTask(usize),
   SetTitle(String),
   SetDescription(String),
   About,
}


pub struct App {
   //items: Vec<i64>,
   counter: i64,
   storage: StorageService,
   database: Database,
   temp_task: Task,
   link: ComponentLink<Self>,
}

impl Component for App {
   type Message = Msg;
   type Properties = ();


   fn create( _: Self::Properties, link: ComponentLink<Self> ) -> Self {
      let storage = StorageService::new( Area::Local ).unwrap();

      let Json( database ) = storage.restore( KEY );

      let database = database.unwrap_or_else( |_| Database::new() );

      return App {
         link,
         //items: Vec::new(),
         counter: 0,
         storage,
         database,
         temp_task: Task::new(),
      };
   }


   fn update( &mut self, msg: Self::Message ) -> ShouldRender {
      match msg {
         Msg::AddTask => {
            if self.temp_task.is_filled_in() {
               self.counter += 1;

               self.database.tasks.push( self.temp_task.clone() );

               self.storage.store( KEY, Json( &self.database ) );

               self.temp_task = Task::new();

               refreshform( "taskform" );  // ignore rust-analyzer unsafe error

               console::info(
                  "New task added and stored in local DB"
               );
            }
         }

         Msg::RemoveTask( ndx ) => {
            self.counter -= if self.counter == 0 { 0 } else { 1 };

            let removed = self.database.tasks.remove( ndx );

            self.storage.remove( KEY );

            self.storage.store( KEY, Json( &self.database ) );

            console::warn( format!( "Removed {:?}", removed ).as_str() );
         }

         Msg::SetTitle( title ) => {
            self.temp_task.title = title;
         }

         Msg::SetDescription( desc ) => {
            self.temp_task.description = desc;
         }

         Msg::About => {
            dialog::alert( "training app" )
         }
      }

      return true;
   }


   fn view( &self ) -> Html {
      let render_item = |(ndx,task): (usize, &Task)| {
         html! {
            <div class="w3-card-4 w3-margin-bottom">
               <header class="w3-orange card-header">
                  <h3>
                     { &task.title }
                  </h3>
               </header>

               <div class="w3-container w3-pale-yellow card-desc">
                  { &task.description }
               </div>

               <footer>
                  <button
                     class="w3-button w3-khaki w3-block w3-hover-shadow w3-hover-orange"
                     onclick=self.link.callback( move |_| Msg::RemoveTask( ndx ) )>
                     { "Remove" }
                  </button>
               </footer>
            </div>
         }
      };

      return html! {
         <div class="app center">
            <h2>{ "Tasks: " }</h2>

            { for self.database.tasks.iter().enumerate().map( render_item ) }

            <div class="w3-card-4 w3-pale-yellow w3-margin-top w3-margin-bottom">
               <form id="taskform" class="form">
                  <label>
                     <input
                        placeholder="Title"
                        oninput=self.link.callback( |e: InputData| Msg::SetTitle( e.value ) )
                     />
                  </label>

                  <label>
                     <textarea
                        rows=2
                        cols=20
                        placeholder="Description"
                        oninput=self.link.callback( |e: InputData| Msg::SetDescription( e.value ) )>
                     </textarea>
                  </label>

                  <button
                     class="w3-button w3-hover-shadow w3-margin-top w3-orange w3-hover-khaki fa fa-plane"
                     onclick=self.link.callback( |_| Msg::AddTask )>
                     { " Add Task" }
                  </button>
               </form>
            </div>

            <button
               class="w3-button w3-khaki w3-hover-orange w3-hover-shadow about"
               onclick=self.link.callback( |_| Msg::About )>
               { "About" }
            </button>
         </div>
      };
   }


   fn change( &mut self, _props: Self::Properties ) -> ShouldRender { true }

   fn rendered( &mut self, _first_render: bool ) {}

   fn destroy( &mut self ) {}
}
