#[cfg(feature = "SiPuppet")]
use leptos::*;
#[cfg(feature = "SiPuppet")]
///This icon requires the feature `SiPuppet` to be enabled.
#[component]
pub fn Puppet(
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
        "M8.984 21.611H6.595v-2.388h2.39zM6.595 2.39h2.39v2.388h-2.39zm13.198 6.028h-5.48l.001-.002-2.941-2.941V0H4.207v7.166h5.48l2.938 2.938.002-.001v3.794l-.003-.003-2.94 2.94H4.207V24h7.166v-5.477l2.94-2.94h5.48V8.417"
        /> < title > { title } < / title > < / svg >
    }
}
