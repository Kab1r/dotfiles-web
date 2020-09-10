use yew::prelude::*;

// Model
#[derive(Properties, Clone, PartialEq)]
pub struct ContentModule {
    pub title: &'static str,
    pub text: &'static str,
    pub icon: Html,
}

// View
impl Component for ContentModule {
    type Message = ();
    type Properties = ContentModule;

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
              <div class="xl:w-1/3 md:w-1/2 p-4">
                  <div class="border border-gray-800 p-6 rounded-lg">
                      <div class="w-10 h-10 inline-flex items-center justify-center rounded-full bg-gray-800 text-indigo-400 mb-4">
                          {self.icon.clone()}
                      </div>
                      <h2 class="text-lg font-medium title-font mb-2">{self.title}</h2>
                      <p class="leading-relaxed text-base">{self.text}</p>
                  </div>
              </div>
        }
    }
}

// Model
#[derive(Properties, Clone, PartialEq)]
pub struct ContentModuleContainer {
    pub children: Children,
}

// View
impl Component for ContentModuleContainer {
    type Message = ();
    type Properties = ContentModuleContainer;
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
            <div class="flex flex-wrap -m-4">
                { self.children.clone() }
            </div>
        }
    }
}
