#[cfg(feature = "FiLogIn")]
use leptos::*;
#[cfg(feature = "FiLogIn")]
///This icon requires the feature `FiLogIn` to be enabled.
#[component]
pub fn LogIn(
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
        "M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4" />< polyline xmlns =
        "http://www.w3.org/2000/svg" points = "10 17 15 12 10 7" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "15" y1 = "12" x2 = "3" y2 = "12" /> < title >
        { title } < / title > < / svg >
    }
}
