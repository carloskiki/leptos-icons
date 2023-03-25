#[cfg(feature = "AiFilledContacts")]
use leptos::*;
#[cfg(feature = "AiFilledContacts")]
///This icon requires the feature `AiFilledContacts` to be enabled.
#[component]
pub fn Contacts(
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
        stroke_witdh = "0" style = style class = "icon" viewBox = "0 0 1024 1024" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M928 224H768v-56c0-4.4-3.6-8-8-8h-56c-4.4 0-8 3.6-8 8v56H548v-56c0-4.4-3.6-8-8-8h-56c-4.4 0-8 3.6-8 8v56H328v-56c0-4.4-3.6-8-8-8h-56c-4.4 0-8 3.6-8 8v56H96c-17.7 0-32 14.3-32 32v576c0 17.7 14.3 32 32 32h832c17.7 0 32-14.3 32-32V256c0-17.7-14.3-32-32-32zM661 736h-43.9c-4.2 0-7.6-3.3-7.9-7.5-3.8-50.6-46-90.5-97.2-90.5s-93.4 40-97.2 90.5c-.3 4.2-3.7 7.5-7.9 7.5H363a8 8 0 0 1-8-8.4c2.8-53.3 32-99.7 74.6-126.1a111.8 111.8 0 0 1-29.1-75.5c0-61.9 49.9-112 111.4-112 61.5 0 111.4 50.1 111.4 112 0 29.1-11 55.5-29.1 75.5 42.7 26.5 71.8 72.8 74.6 126.1.4 4.6-3.2 8.4-7.8 8.4zM512 474c-28.5 0-51.7 23.3-51.7 52s23.2 52 51.7 52c28.5 0 51.7-23.3 51.7-52s-23.2-52-51.7-52z"
        /> < title > { title } < / title > < / svg >
    }
}
