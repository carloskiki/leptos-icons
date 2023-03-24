#[cfg(feature = "BiSolidWine")]
use leptos::*;
#[cfg(feature = "BiSolidWine")]
///This icon requires the feature `BiSolidWine` to be enabled.
#[component]
pub fn Wine(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M11 17.916V20H9v2h6v-2h-2v-2.084c3.162-.402 5.849-2.66 6.713-5.793.264-.952.312-2.03.143-3.206l-.866-6.059A1 1 0 0 0 18 2H6a1 1 0 0 0-.99.858l-.865 6.058c-.169 1.177-.121 2.255.143 3.206.863 3.134 3.55 5.392 6.712 5.794zM17.133 4l.57 4H6.296l.571-4h10.266z"
        /> < title > { title } < / title > < / svg >
    }
}
