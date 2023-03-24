#[cfg(feature = "SiAerospike")]
use leptos::*;
#[cfg(feature = "SiAerospike")]
///This icon requires the feature `SiAerospike` to be enabled.
#[component]
pub fn Aerospike(
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
        "M0 0v24h24V0zm19.295 5.386v1.64l-3.576 1.586v7.363l3.576 1.602v1.565L5.672 12.98l-1.607-.688 1.607-.743zm-4.948 3.825L7.45 12.283l6.897 3.092Z"
        /> < title > { title } < / title > < / svg >
    }
}
