#[cfg(feature = "IoApps")]
use leptos::*;
#[cfg(feature = "IoApps")]
///This icon requires the feature `IoApps` to be enabled.
#[component]
pub fn Apps(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M104,160a56,56,0,1,1,56-56A56.06,56.06,0,0,1,104,160Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M256,160a56,56,0,1,1,56-56A56.06,56.06,0,0,1,256,160Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M408,160a56,56,0,1,1,56-56A56.06,56.06,0,0,1,408,160Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M104,312a56,56,0,1,1,56-56A56.06,56.06,0,0,1,104,312Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M256,312a56,56,0,1,1,56-56A56.06,56.06,0,0,1,256,312Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M408,312a56,56,0,1,1,56-56A56.06,56.06,0,0,1,408,312Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M104,464a56,56,0,1,1,56-56A56.06,56.06,0,0,1,104,464Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M256,464a56,56,0,1,1,56-56A56.06,56.06,0,0,1,256,464Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M408,464a56,56,0,1,1,56-56A56.06,56.06,0,0,1,408,464Z" /> < title > { title } <
        / title > < / svg >
    }
}
