#[cfg(feature = "TiVolume")]
use leptos::*;
#[cfg(feature = "TiVolume")]
///This icon requires the feature `TiVolume` to be enabled.
#[component]
pub fn Volume(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.138 5.824c-.449 0-.905.152-1.356.453l-2.672 1.781c-.753.503-2.206.942-3.11.942-1.654 0-3 1.346-3 3v2c0 1.654 1.346 3 3 3 .904 0 2.357.439 3.109.941l2.672 1.781c.451.301.907.453 1.356.453.898.001 1.863-.68 1.863-2.175v-10c0-1.495-.965-2.176-1.862-2.176zm-3.138 10.322c-1.093-.651-2.789-1.146-4-1.146-.552 0-1-.448-1-1v-2c0-.552.448-1 1-1 1.211 0 2.907-.495 4-1.146v6.292zm3 1.854l-.006.12-.104-.062-1.89-1.26v-7.596l1.891-1.261.104-.062.005.121v10z"
        /> < title > { title } < / title > < / svg >
    }
}
