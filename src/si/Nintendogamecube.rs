#[cfg(feature = "SiNintendogamecube")]
use leptos::*;
#[cfg(feature = "SiNintendogamecube")]
///This icon requires the feature `SiNintendogamecube` to be enabled.
#[component]
pub fn Nintendogamecube(
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
        "M6.816 15.126l4.703 2.715v-5.433L6.814 9.695v5.432zm-2.025 1.168l6.73 3.882v3.82L1.481 18.206V6.616l3.31 1.91v7.769zM12 6.145L7.298 8.863 12 11.579l4.704-2.717L12 6.146zm0-2.332l5.659 3.274 3.31-1.91L12 0 1.975 5.79 5.28 7.695zm7.207 12.48v-3.947l-2.023 1.167v1.614l-4.703 2.715v.005-5.436L22.518 6.62v11.587L12.48 24v-3.817l6.727-3.887z"
        /> < title > { title } < / title > < / svg >
    }
}
