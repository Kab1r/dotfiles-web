#![recursion_limit="256"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;
mod components;
mod pages;
use components::{ContentModule, FontAwesomeIcon};
use pages::Index;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Index>::new().mount_as_body_with_props(
        Index {
            title: "Kabir Kwatra's Dotfiles",
            tagline: "Chezmoi | Fish | Doom Emacs",
            script: "curl -sSL https://dotfiles.kwatra.me/setup.sh | sh",
            github_url: "https://github.com/KabirKwatra/DotFiles",
            gitlab_url: "https://gitlab.com/KabirKwatra/DotFiles",
            sourcehut_url: "https://sr.ht/~kabir/dotfiles",
            content_modules: vec![
                ContentModule {
                    title: "Powerful",
                    text: "Customized Doom flavored Emacs using Vim bindings.",
                    icon: FontAwesomeIcon { icon: vec!["fas", "fa-bolt", "fa-lg"] }.render()
                },
                ContentModule {
                    title: "Elegant",
                    text: "Fish Shell with Starship prompt offers balance of performance and modern features.",
                    icon: FontAwesomeIcon { icon: vec!["fas", "fa-pen-fancy", "fa-lg"] }.render()
                },
                ContentModule {
                    title: "Portable",
                    text: "Easy single command installation on all common platforms.",
                    icon: FontAwesomeIcon { icon: vec!["far", "fa-share-square", "fa-lg"] }.render()
                },
                ContentModule {
                    title: "Organized",
                    text: "Fully Managed using Chezmoi.",
                    icon: FontAwesomeIcon { icon: vec!["fas", "fa-sitemap", "fa-lg"] }.render()
                },
            ],
    });
}
