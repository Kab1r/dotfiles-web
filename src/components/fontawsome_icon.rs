use yew::prelude::*;

// Model
#[derive(Properties, Clone, PartialEq)]
pub struct FontAwesomeIcon {
    pub icon: Vec<&'static str>,
}

// View
impl Component for FontAwesomeIcon {
    type Message = ();
    type Properties = FontAwesomeIcon;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <a class={self.icon.iter().fold("".to_string(), |acc, class| acc + " " + class)} />
        }
    }
}
