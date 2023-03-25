#[cfg(feature = "CgMailReply")]
use leptos::*;
#[cfg(feature = "CgMailReply")]
///This icon requires the feature `CgMailReply` to be enabled.
#[component]
pub fn MailReply(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    let style = format!("{} color: {};", style, color);
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.3623 15.529L8.94804 16.9432L3.99829 11.9934L8.94804 7.0437L10.3623 8.45791L7.86379 10.9564H16.0018C18.2109 10.9564 20.0018 12.7472 20.0018 14.9564V16.9564H18.0018V14.9564C18.0018 13.8518 17.1063 12.9564 16.0018 12.9564H7.78965L10.3623 15.529Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
