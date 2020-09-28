use crate::components::*;
use yew::prelude::*;

// Model
#[derive(Properties, Clone, PartialEq)]
pub struct Index {
    pub title: &'static str,
    pub tagline: &'static str,
    pub script: &'static str,
    pub github_url: &'static str,
    pub gitlab_url: &'static str,
    pub sourcehut_url: &'static str,
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
                    <div class="flex flex-wrap mb-2 gap-x-2 flex-row items-center mt-5">
                        <GitButton button_href={self.github_url}>
                            <FontAwesomeIcon icon=vec!["fab", "fa-github-alt", "fa-3x"] />
                        </GitButton>
                        <GitButton button_href={self.gitlab_url}>
                            <FontAwesomeIcon icon=vec!["fab", "fa-gitlab", "fa-3x"] />
                        </GitButton>
                        <GitButton button_href={self.sourcehut_url}>
                            <FontAwesomeIcon icon=vec!["far", "fa-circle", "fa-3x"] />
                        </GitButton>
                    </div>
                </div>
                <ContentModuleContainer>
                    { for self.content_modules.iter().map(|item| item.render())}
                </ContentModuleContainer>
            </Body>
        }
    }
}
