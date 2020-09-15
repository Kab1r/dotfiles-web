use wasm_bindgen::prelude::*;
use yew::prelude::*;
mod components;
mod pages;
use components::ContentModule;
use pages::Index;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Index>::new().mount_as_body_with_props(
        Index {
            title: "Kabir Kwatra's Dotfiles",
            tagline: "Chezmoi | Fish | Doom Emacs",
            script: "curl -sSL https://dotfiles.kwatra.me/setup.sh | sh",
            github_url: "https://github.com/KabirKwatra/DotFiles",
            content_modules: vec![
                ContentModule {
                    title: "Powerful",
                    text: "Customized Doom flavored Emacs using Vim bindings.",
                    icon: html! {}
                },
                ContentModule {
                    title: "Elegant",
                    text: "Fish Shell with Starship prompt offers balance of performance and modern features.",
                    icon: html! {}
                },
                ContentModule {
                    title: "Portable",
                    text: "Easy single command installation on all common platforms.",
                    icon: html! {}
                },
                ContentModule {
                    title: "Organized",
                    text: "Fully Managed using Chezmoi.",
                    icon: html! {}
                },
            ],
    });
}
