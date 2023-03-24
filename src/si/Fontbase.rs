#[cfg(feature = "SiFontbase")]
use leptos::*;
#[cfg(feature = "SiFontbase")]
///This icon requires the feature `SiFontbase` to be enabled.
#[component]
pub fn Fontbase(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M23.079 13.996c-2.702-2.771-5.702-5.703-8.105-8.103-1.62-1.621-4.284-1.621-5.943 0-2.97 2.963-5.248 5.21-8.104 8.066a3.12 3.12 0 0 0 0 4.437 3.12 3.12 0 0 0 4.437 0l2.2-2.2 2.2 2.2a3.12 3.12 0 0 0 4.438 0 3.12 3.12 0 0 0 0-4.438l4.4 4.4a3.12 3.12 0 0 0 4.438 0c1.274-1.16 1.274-3.165.039-4.362z"
        /> < title > { title } < / title > < / svg >
    }
}
