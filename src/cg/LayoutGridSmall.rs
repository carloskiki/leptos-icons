#[cfg(feature = "CgLayoutGridSmall")]
use leptos::*;
#[cfg(feature = "CgLayoutGridSmall")]
///This icon requires the feature `CgLayoutGridSmall` to be enabled.
#[component]
pub fn LayoutGridSmall(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 7H9V9H7V7Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 7H13V9H11V7Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M17 7H15V9H17V7Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 11H9V13H7V11Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 11H11V13H13V11Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M15 11H17V13H15V11Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 15H7V17H9V15Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 15H13V17H11V15Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M17 15H15V17H17V15Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
