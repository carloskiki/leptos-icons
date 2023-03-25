#[cfg(feature = "SiQantas")]
use leptos::*;
#[cfg(feature = "SiQantas")]
///This icon requires the feature `SiQantas` to be enabled.
#[component]
pub fn Qantas(
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
        "M0 3.47l.218.572c1.925 5.006 5.566 2.689 10.415 7.139l.056.05c.652.599 1.1.044.888-.306a.76.76 0 0 1-.165-.532 6.7 6.7 0 0 1 2.606 1.369l-.06.126c-.366.73-3.959.421-4 1.943a.969.969 0 0 0 .607.923l.71.287a17.34 17.34 0 0 1 6.086 4.146.086.086 0 0 1-.063.147.079.079 0 0 1-.054-.018 17.32 17.32 0 0 0-8.173-3.61.467.467 0 0 1-.39-.41c-.548-5.089-5.575-5.434-7.492-8.705l5.313 13.94H24L9.979 6.449a10.022 10.022 0 0 0-7.045-2.98Z"
        /> < title > { title } < / title > < / svg >
    }
}
