#[cfg(feature = "RiHealthFillEmpathize")]
use leptos::*;
#[cfg(feature = "RiHealthFillEmpathize")]
///This icon requires the feature `RiHealthFillEmpathize` to be enabled.
#[component]
pub fn Empathize(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M18.364 10.98c1.562 1.561 1.562 4.094 0 5.656l-5.657 5.657c-.39.39-1.024.39-1.414 0l-5.657-5.657c-1.562-1.562-1.562-4.095 0-5.657 1.562-1.562 4.095-1.562 5.657 0l.706.707.708-.707c1.562-1.562 4.095-1.562 5.657 0zM12 1c2.21 0 4 1.79 4 4s-1.79 4-4 4-4-1.79-4-4 1.79-4 4-4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
