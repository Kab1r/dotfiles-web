use yew::prelude::*;
// Model
#[derive(Properties, Clone, PartialEq)]
pub struct CodeButton {
    pub script: &'static str,
}
fn copy_to_clipboard(script: &str) {
    let _ = yew::web_sys::window()
        .expect("Window Not Available")
        .navigator()
        .clipboard()
        .write_text(script);
}
// View
impl Component for CodeButton {
    type Message = ();
    type Properties = CodeButton;
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
        let script = <&str>::clone(&self.script);
        let code_click_callback = Callback::from(move |_| copy_to_clipboard(script));
        html! {
            <button onclick=code_click_callback class="flex mx-auto mt-2 text-white bg-gray-700 active:bg-indigo-600 border-0 py-2 px-4 focus:outline-none rounded text-xs ">
                <code class="language-shell">
                    {self.script}
                </code>
            </button>
        }
    }
}
