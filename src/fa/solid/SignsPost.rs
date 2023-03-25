#[cfg(feature = "FaSolidSignsPost")]
use leptos::*;
#[cfg(feature = "FaSolidSignsPost")]
///This icon requires the feature `FaSolidSignsPost` to be enabled.
#[component]
pub fn SignsPost(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M218 32H58C40.3 32 26 46.3 26 64v64c0 17.7 14.3 32 32 32H435.4c4.2 0 8.3-1.7 11.3-4.7l48-48c6.2-6.2 6.2-16.4 0-22.6l-48-48c-3-3-7.1-4.7-11.3-4.7H282c0-17.7-14.3-32-32-32s-32 14.3-32 32zM474 256c0-17.7-14.3-32-32-32H282V192H218v32H64.6c-4.2 0-8.3 1.7-11.3 4.7l-48 48c-6.2 6.2-6.2 16.4 0 22.6l48 48c3 3 7.1 4.7 11.3 4.7H442c17.7 0 32-14.3 32-32V256zM282 480V384H218v96c0 17.7 14.3 32 32 32s32-14.3 32-32z"
        /> < title > { title } < / title > < / svg >
    }
}
