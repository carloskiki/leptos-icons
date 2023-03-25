#[cfg(feature = "BsSafeFill")]
use leptos::*;
#[cfg(feature = "BsSafeFill")]
///This icon requires the feature `BsSafeFill` to be enabled.
#[component]
pub fn SafeFill(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-safe-fill" viewBox = "0 0 16 16" width = size.clone() height =
        size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M9.778 9.414A2 2 0 1 1 6.95 6.586a2 2 0 0 1 2.828 2.828z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M2.5 0A1.5 1.5 0 0 0 1 1.5V3H.5a.5.5 0 0 0 0 1H1v3.5H.5a.5.5 0 0 0 0 1H1V12H.5a.5.5 0 0 0 0 1H1v1.5A1.5 1.5 0 0 0 2.5 16h12a1.5 1.5 0 0 0 1.5-1.5v-13A1.5 1.5 0 0 0 14.5 0h-12zm3.036 4.464 1.09 1.09a3.003 3.003 0 0 1 3.476 0l1.09-1.09a.5.5 0 1 1 .707.708l-1.09 1.09c.74 1.037.74 2.44 0 3.476l1.09 1.09a.5.5 0 1 1-.707.708l-1.09-1.09a3.002 3.002 0 0 1-3.476 0l-1.09 1.09a.5.5 0 1 1-.708-.708l1.09-1.09a3.003 3.003 0 0 1 0-3.476l-1.09-1.09a.5.5 0 1 1 .708-.708zM14 6.5v3a.5.5 0 0 1-1 0v-3a.5.5 0 0 1 1 0z"
        /> < title > { title } < / title > < / svg >
    }
}
