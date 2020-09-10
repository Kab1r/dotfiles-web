use yew::prelude::*;

// Model
#[derive(Properties, Clone, PartialEq)]
pub struct Heading {
    pub title: &'static str,
    pub tagline: &'static str,
}

// View
impl Component for Heading {
    type Message = ();
    type Properties = Heading;
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
            <div class="flex flex-wrap w-full mb-6 flex-col items-center text-center">
                <h1 class="sm:text-4xl text-3xl font-medium title-font mb-2 text-gray light-mode:text-dark-light">
                    {self.title}
                </h1>
                <p class="lg:w-1/2 w-full leading-relaxed text-base">
                    {self.tagline}
                </p>
            </div>
        }
    }
}
