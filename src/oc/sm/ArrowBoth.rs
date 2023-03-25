#[cfg(feature = "OcSmArrowBoth")]
use leptos::*;
#[cfg(feature = "OcSmArrowBoth")]
///This icon requires the feature `OcSmArrowBoth` to be enabled.
#[component]
pub fn ArrowBoth(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M3.72 3.72a.751.751 0 0 1 1.042.018.751.751 0 0 1 .018 1.042L2.56 7h10.88l-2.22-2.22a.751.751 0 0 1 .018-1.042.751.751 0 0 1 1.042-.018l3.5 3.5a.75.75 0 0 1 0 1.06l-3.5 3.5a.749.749 0 0 1-1.275-.326.749.749 0 0 1 .215-.734l2.22-2.22H2.56l2.22 2.22a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215l-3.5-3.5a.75.75 0 0 1 0-1.06Z"
        /> < title > { title } < / title > < / svg >
    }
}
