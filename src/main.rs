use rs_covid19gr::cum::get_total_data;
use yew::prelude::*;

struct App {
    link: ComponentLink<Self>,
    total_cases: u32,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // let total = get_total_data();
        // let tests =
        Self {
            link,
            total_cases: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let text_out: &str = "Covid19gr Tests";
        html! {
            <div>
                <h2>{ text_out   } </h2>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
