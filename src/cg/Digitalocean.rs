#[cfg(feature = "CgDigitalocean")]
use leptos::*;
#[cfg(feature = "CgDigitalocean")]
///This icon requires the feature `CgDigitalocean` to be enabled.
#[component]
pub fn Digitalocean(
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
        "M12 6C8.68629 6 6 8.68629 6 12H1C1 5.92487 5.92487 1 12 1C18.0751 1 23 5.92487 23 12C23 18.0751 18.0751 23 12 23V18C15.3137 18 18 15.3137 18 12C18 8.68629 15.3137 6 12 6Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 18V13H12V18H7Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 18V22H7V18H3Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M3 18H1V16H3V18Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
