#[cfg(feature = "SiQwiklabs")]
use leptos::*;
#[cfg(feature = "SiQwiklabs")]
///This icon requires the feature `SiQwiklabs` to be enabled.
#[component]
pub fn Qwiklabs(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M14.346 18.205A6.464 6.464 0 0 0 12 5.72a6.462 6.462 0 0 0-2.346 12.485.69.69 0 0 0 .961-.623v-5.4a1.385 1.385 0 1 1 2.77 0v5.4a.692.692 0 0 0 .961.623zm.809 5.558C20.252 22.378 24 17.718 24 12.182c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.536 3.748 10.196 8.845 11.581a.7.7 0 0 0 .049.013l.059.016.001-.002a1.385 1.385 0 0 0 .635-2.695 9.231 9.231 0 1 1 4.824-.001 1.385 1.385 0 0 0 .635 2.695l.001.002.059-.016.049-.013z"
        /> < title > { title } < / title > < / svg >
    }
}
