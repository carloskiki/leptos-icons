#[cfg(feature = "ImMug")]
use leptos::*;
#[cfg(feature = "ImMug")]
///This icon requires the feature `ImMug` to be enabled.
#[component]
pub fn Mug(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M15 5h-3v-1.5c0-1.381-2.686-2.5-6-2.5s-6 1.119-6 2.5v10c0 1.381 2.686 2.5 6 2.5s6-1.119 6-2.5v-1.5h3c0.552 0 1-0.448 1-1v-5c0-0.552-0.448-1-1-1zM2.751 4.037c-0.578-0.19-0.928-0.394-1.116-0.537 0.188-0.143 0.538-0.347 1.116-0.537 0.905-0.298 2.059-0.463 3.249-0.463s2.344 0.164 3.249 0.463c0.578 0.19 0.928 0.394 1.116 0.537-0.188 0.143-0.538 0.347-1.116 0.537-0.905 0.298-2.059 0.463-3.249 0.463s-2.344-0.164-3.249-0.463zM14 10h-2v-3h2v3z"
        /> < title > { title } < / title > < / svg >
    }
}
