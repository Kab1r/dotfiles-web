use crate::components::*;
use yew::prelude::*;

// Model
#[derive(Properties, Clone, PartialEq)]
pub struct Index {
    pub title: &'static str,
    pub tagline: &'static str,
    pub script: &'static str,
    pub github_url: &'static str,
    pub content_modules: Vec<ContentModule>,
}

// View
impl Component for Index {
    type Message = ();
    type Properties = Index;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
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
            <Body>
                <Heading title={self.title} tagline={self.tagline} />
                <div class="flex flex-wrap w-full mb-5 flex-col items-center text-center">
                    <small>{"Quick Install"}</small>
                    <CodeButton script={self.script}/>
                    <GitButton button_href={self.github_url}>
                        {"Github"}
                    </GitButton>
                </div>
                <ContentModuleContainer>
                    { for self.content_modules.iter().map(|item| item.render())}
                </ContentModuleContainer>
            </Body>
        }
    }
}
