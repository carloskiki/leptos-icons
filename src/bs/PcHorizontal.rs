#[cfg(feature = "BsPcHorizontal")]
use leptos::*;
#[cfg(feature = "BsPcHorizontal")]
///This icon requires the feature `BsPcHorizontal` to be enabled.
#[component]
pub fn PcHorizontal(
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
        class = "bi bi-pc-horizontal" viewBox = "0 0 16 16" width = size.clone() height =
        size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M1 6a1 1 0 0 0-1 1v3a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1V7a1 1 0 0 0-1-1H1Zm11.5 1a.5.5 0 1 1 0 1 .5.5 0 0 1 0-1Zm2 0a.5.5 0 1 1 0 1 .5.5 0 0 1 0-1ZM1 7.5a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5ZM1.25 9h5.5a.25.25 0 0 1 0 .5h-5.5a.25.25 0 0 1 0-.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
