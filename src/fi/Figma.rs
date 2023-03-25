#[cfg(feature = "FiFigma")]
use leptos::*;
#[cfg(feature = "FiFigma")]
///This icon requires the feature `FiFigma` to be enabled.
#[component]
pub fn Figma(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 5.5A3.5 3.5 0 0 1 8.5 2H12v7H8.5A3.5 3.5 0 0 1 5 5.5z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 2h3.5a3.5 3.5 0 1 1 0 7H12V2z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12.5a3.5 3.5 0 1 1 7 0 3.5 3.5 0 1 1-7 0z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M5 19.5A3.5 3.5 0 0 1 8.5 16H12v3.5a3.5 3.5 0 1 1-7 0z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M5 12.5A3.5 3.5 0 0 1 8.5 9H12v7H8.5A3.5 3.5 0 0 1 5 12.5z" /> < title > { title
        } < / title > < / svg >
    }
}
