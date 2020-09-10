#![recursion_limit = "256"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;
mod components;
use components::*;

pub struct Index {
    title: &'static str,
    tagline: &'static str,
    script: &'static str,
    button_href: &'static str,
    button_text: &'static str,
    content_modules: Vec<ContentModule>,
}

impl Component for Index {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Index {
            title: "Kabir Kwatra's Dotfiles",
            tagline: "Chezmoi | Fish | Doom Emacs",
            script: "curl -sSL https://dotfiles.kwatra.me/setup.sh | sh",
            button_href: "https://github.com/KabirKwatra/DotFiles",
            button_text: "Github",
            content_modules: vec![
                ContentModule { title: "Powerful",  text: "Customized Doom flavored Emacs using Vim bindings.",                                  icon: html! {} },
                ContentModule { title: "Elegant",   text: "Fish Shell with Starship prompt offers balance of performance and modern features.",  icon: html! {} },
                ContentModule { title: "Portable",  text: "Easy single command installation on all common platforms.",                           icon: html! {} },
                ContentModule { title: "Organized", text: "Fully Managed using Chezmoi.",                                                        icon: html! {} },
            ],
        }
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
                        <Heading title={self.title} tagline={self.tagline} />
                        <div class="flex flex-wrap w-full mb-5 flex-col items-center text-center">
                            <small>{"Quick Install"}</small>
                                <CodeButton script={self.script}/>
                            <a href={self.button_href}>
                                <button class="flex mx-auto mt-5 text-white bg-indigo-500 border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600 rounded text-lg">
                                    {self.button_text}
                                </button>
                            </a>
                        </div>
                        <ContentModuleContainer>
                            { for self.content_modules.iter().map(|item| item.render())}
                        </ContentModuleContainer>
                    </div>
                </section>
            </body>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Index>::new().mount_as_body();
}
