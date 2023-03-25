#[cfg(feature = "BiSolidTShirt")]
use leptos::*;
#[cfg(feature = "BiSolidTShirt")]
///This icon requires the feature `BiSolidTShirt` to be enabled.
#[component]
pub fn TShirt(
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
        xmlns = "http://www.w3.org/2000/svg" d =
        "M21.316 4.055C19.556 3.478 15 1.985 15 2a3 3 0 1 1-6 0c0-.015-4.556 1.478-6.317 2.055A.992.992 0 0 0 2 5.003v3.716a1 1 0 0 0 1.242.97L6 9v12a1 1 0 0 0 1 1h10a1 1 0 0 0 1-1V9l2.758.689A1 1 0 0 0 22 8.719V5.003a.992.992 0 0 0-.684-.948z"
        /> < title > { title } < / title > < / svg >
    }
}
