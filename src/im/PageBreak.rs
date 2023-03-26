#[cfg(feature = "ImPageBreak")]
use leptos::*;
#[cfg(feature = "ImPageBreak")]
///This icon requires the feature `ImPageBreak` to be enabled.
#[component]
pub fn PageBreak(
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
        "M0 8h2v1h-2zM3 8h3v1h-3zM7 8h2v1h-2zM10 8h3v1h-3zM14 8h2v1h-2zM13.75 0l0.25 7h-12l0.25-7h0.5l0.25 6h10l0.25-6zM2.25 16l-0.25-6h12l-0.25 6h-0.5l-0.25-5h-10l-0.25 5z"
        /> < title > { title } < / title > < / svg >
    }
}
