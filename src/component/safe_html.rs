use yew::{function_component, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SafeHtmlProperties {
    pub html: String,
}

#[function_component(SafeHtml)]
pub fn safe_html(props: &SafeHtmlProperties) -> Html {
    let div = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&props.html.clone());

    Html::VRef(div.into())
}
