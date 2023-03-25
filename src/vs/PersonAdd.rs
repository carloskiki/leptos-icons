#[cfg(feature = "VsPersonAdd")]
use leptos::*;
#[cfg(feature = "VsPersonAdd")]
///This icon requires the feature `VsPersonAdd` to be enabled.
#[component]
pub fn PersonAdd(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M13 10h-1v2h-2v1h2v2h1v-2h2v-1h-2v-2zM8.556 2.169a1 1 0 1 0-1.112 1.663 1 1 0 0 0 1.112-1.663zm-1.667-.832A2 2 0 1 1 9.11 4.663a2 2 0 0 1-2.22-3.326zM6.77 5.49h2.46A1.77 1.77 0 0 1 11 7.26V8h-1v-.74a.76.76 0 0 0-.77-.77H6.77a.76.76 0 0 0-.77.77V10h1v3.31a.2.2 0 0 0 .2.2H8v1.02h-.8a1.2 1.2 0 0 1-1.2-1.2V11a1.06 1.06 0 0 1-1-1.1V7.26a1.77 1.77 0 0 1 1.77-1.77z"
        /> < title > { title } < / title > < / svg >
    }
}
