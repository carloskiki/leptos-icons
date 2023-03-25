#[cfg(feature = "VsLinkExternal")]
use leptos::*;
#[cfg(feature = "VsLinkExternal")]
///This icon requires the feature `VsLinkExternal` to be enabled.
#[component]
pub fn LinkExternal(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M1.5 1H6v1H2v12h12v-4h1v4.5l-.5.5h-13l-.5-.5v-13l.5-.5z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15 1.5V8h-1V2.707L7.243 9.465l-.707-.708L13.293 2H8V1h6.5l.5.5z" /> < title > {
        title } < / title > < / svg >
    }
}
