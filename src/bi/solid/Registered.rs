#[cfg(feature = "BiSolidRegistered")]
use leptos::*;
#[cfg(feature = "BiSolidRegistered")]
///This icon requires the feature `BiSolidRegistered` to be enabled.
#[component]
pub fn Registered(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d = "M13 9h-3v2h3a1 1 0 0 0 0-2z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2zm2.13 15-2.67-4H10v4H8V7h5a3 3 0 0 1 .79 5.88L16.54 17z"
        /> < title > { title } < / title > < / svg >
    }
}
