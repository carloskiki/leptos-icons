#[cfg(feature = "SiCraftcms")]
use leptos::*;
#[cfg(feature = "SiCraftcms")]
///This icon requires the feature `SiCraftcms` to be enabled.
#[component]
pub fn Craftcms(
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
        "M21.474 0H2.526A2.516 2.516 0 0 0 0 2.526v18.948A2.516 2.516 0 0 0 2.526 24h18.948A2.534 2.534 0 0 0 24 21.474V2.526A2.516 2.516 0 0 0 21.474 0m-9.516 14.625c.786 0 1.628-.31 2.442-1.039l1.123 1.291c-1.18.955-2.527 1.488-3.874 1.488-2.667 0-4.35-1.769-3.958-4.267.393-2.498 2.667-4.266 5.334-4.266 1.29 0 2.498.505 3.34 1.431l-1.572 1.291c-.45-.59-1.207-.982-2.05-.982-1.6 0-2.834 1.039-3.087 2.526-.224 1.488.674 2.527 2.302 2.527"
        /> < title > { title } < / title > < / svg >
    }
}
