use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self{
        Model{
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender{
        match msg{
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender{
        false
    }

    fn view(&self) -> Html{
        html!{
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }


}

fn main() {

    yew::start_app::<Model>();


}
