#[cfg(feature = "ImArrowDownRight2")]
use leptos::*;
#[cfg(feature = "ImArrowDownRight2")]
///This icon requires the feature `ImArrowDownRight2` to be enabled.
#[component]
pub fn ArrowDownRight2(
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
        "M2.293 3.707l8.293 8.293h-3.586c-0.552 0-1 0.448-1 1s0.448 1 1 1h6c0.404 0 0.769-0.244 0.924-0.617 0.051-0.124 0.076-0.254 0.076-0.383h0.001v-6c0-0.552-0.448-1-1-1s-1 0.448-1 1v3.586l-8.293-8.293c-0.195-0.195-0.451-0.293-0.707-0.293s-0.512 0.098-0.707 0.293c-0.391 0.39-0.391 1.024 0 1.414z"
        /> < title > { title } < / title > < / svg >
    }
}
