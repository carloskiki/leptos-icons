#[cfg(feature = "BsSnow3")]
use leptos::*;
#[cfg(feature = "BsSnow3")]
///This icon requires the feature `BsSnow3` to be enabled.
#[component]
pub fn Snow3(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-snow3" viewBox = "0 0 16 16" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 7.5a.5.5 0 1 0 0 1 .5.5 0 0 0 0-1z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M8 16a.5.5 0 0 1-.5-.5v-1.293l-.646.647a.5.5 0 0 1-.707-.708L7.5 12.793v-1.51l-2.053-1.232-1.348.778-.495 1.85a.5.5 0 1 1-.966-.26l.237-.882-1.12.646a.5.5 0 0 1-.5-.866l1.12-.646-.883-.237a.5.5 0 1 1 .258-.966l1.85.495L5 9.155v-2.31l-1.4-.808-1.85.495a.5.5 0 1 1-.259-.966l.884-.237-1.12-.646a.5.5 0 0 1 .5-.866l1.12.646-.237-.883a.5.5 0 1 1 .966-.258l.495 1.849 1.348.778L7.5 4.717v-1.51L6.147 1.854a.5.5 0 1 1 .707-.708l.646.647V.5a.5.5 0 0 1 1 0v1.293l.647-.647a.5.5 0 1 1 .707.708L8.5 3.207v1.51l2.053 1.232 1.348-.778.495-1.85a.5.5 0 1 1 .966.26l-.236.882 1.12-.646a.5.5 0 0 1 .5.866l-1.12.646.883.237a.5.5 0 1 1-.26.966l-1.848-.495-1.4.808v2.31l1.4.808 1.849-.495a.5.5 0 1 1 .259.966l-.883.237 1.12.646a.5.5 0 0 1-.5.866l-1.12-.646.236.883a.5.5 0 1 1-.966.258l-.495-1.849-1.348-.778L8.5 11.283v1.51l1.354 1.353a.5.5 0 0 1-.707.708l-.647-.647V15.5a.5.5 0 0 1-.5.5zm2-6.783V6.783l-2-1.2-2 1.2v2.434l2 1.2 2-1.2z"
        /> < title > { title } < / title > < / svg >
    }
}
