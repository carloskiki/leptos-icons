#[cfg(feature = "BiSolidDog")]
use leptos::*;
#[cfg(feature = "BiSolidDog")]
///This icon requires the feature `BiSolidDog` to be enabled.
#[component]
pub fn Dog(
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
        "M21 6h-2l-1.27-1.27A2.49 2.49 0 0 0 16 4h-2.5A2.64 2.64 0 0 0 11 2v6.36a4.38 4.38 0 0 0 1.13 2.72 6.57 6.57 0 0 0 4.13 1.82l3.45-1.38a3 3 0 0 0 1.73-1.84L22 8.15a1.06 1.06 0 0 0 0-.31V7a1 1 0 0 0-1-1zm-5 2a1 1 0 1 1 1-1 1 1 0 0 1-1 1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.38 11.74A5.24 5.24 0 0 1 10.07 9H6a1.88 1.88 0 0 1-2-2 1 1 0 0 0-2 0 4.69 4.69 0 0 0 .48 2A3.58 3.58 0 0 0 4 10.53V22h3v-5h6v5h3v-8.13a7.35 7.35 0 0 1-4.62-2.13z"
        /> < title > { title } < / title > < / svg >
    }
}
