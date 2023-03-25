#[cfg(feature = "SiGooglelens")]
use leptos::*;
#[cfg(feature = "SiGooglelens")]
///This icon requires the feature `SiGooglelens` to be enabled.
#[component]
pub fn Googlelens(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 16.667a4.666 4.666 0 100-9.333 4.666 4.666 0 000 9.333m8 6a2.666 2.666 0 100-5.333 2.666 2.666 0 000 5.333m-13.333-2a3.343 3.343 0 01-3.334-3.334v-2.666H0v2.666A6.665 6.665 0 006.667 24h2.666v-3.333zm-3.334-14c0-1.834 1.5-3.334 3.334-3.334h2.666V0H6.667A6.665 6.665 0 000 6.667v2.666h3.333zm14-3.334c1.834 0 3.334 1.5 3.334 3.334v2.666H24V6.667A6.665 6.665 0 0017.333 0h-2.666v3.333Z"
        /> < title > { title } < / title > < / svg >
    }
}
