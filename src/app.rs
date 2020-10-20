use yew::prelude::*;


pub enum Msg {
  AddOne,
  RemoveOne,
}


pub struct App {
  counter: i64,
  link: ComponentLink<Self>,
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create( _: Self::Properties, link: ComponentLink<Self> ) -> Self {
    return App {
      link,
      counter: 0,
    };
  }


  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::AddOne => self.counter += 1,
      Msg::RemoveOne => self.counter -= 1,
    }

    return true;
  }


  fn view( &self ) -> Html {
    return html! {
      <div>
        <p> {"Counter: "} { self.counter } </p>
        <button onclick=self.link.callback( |_| Msg::AddOne )>{ "+" }</button>
        <button onclick=self.link.callback( |_| Msg::RemoveOne )>{ "-" }</button>
      </div>
    };
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    return true;
  }
}
