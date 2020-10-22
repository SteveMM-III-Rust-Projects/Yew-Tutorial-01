use yew::prelude::*;
use rand::prelude::*;


pub enum Msg {
  AddOne,
  RemoveOne,
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


  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::AddOne => {self.counter += 1; self.items.push( random() );},
      Msg::RemoveOne => {
        self.counter -= if self.counter == 0 {0} else {1};
        self.items.pop();
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
        <button onclick=self.link.callback( |_| Msg::AddOne )>{ "+" }</button>
        <button onclick=self.link.callback( |_| Msg::RemoveOne )>{ "-" }</button>
      </div>
    };
  }


  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    return true;
  }
}
