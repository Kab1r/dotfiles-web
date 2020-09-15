use yew::prelude::*;

// Model
#[derive(Properties, Clone, PartialEq)]
pub struct Body {
    pub children: Children,
}

// View
impl Component for Body {
    type Message = ();
    type Properties = Body;
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
            <body>
                <section class="text-gray-300 bg-gray-900 min-h-screen">
                    <div class="container px-5 py-12 mx-auto">
                        {self.children.clone()}
                    </div>
                </section>
            </body>
        }
    }
}
