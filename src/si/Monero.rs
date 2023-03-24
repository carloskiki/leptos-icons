#[cfg(feature = "SiMonero")]
use leptos::*;
#[cfg(feature = "SiMonero")]
///This icon requires the feature `SiMonero` to be enabled.
#[component]
pub fn Monero(
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
        "M12 0C5.365 0 0 5.373 0 12.015c0 1.335.228 2.607.618 3.81h3.577V5.729L12 13.545l7.805-7.815v10.095h3.577c.389-1.203.618-2.475.618-3.81C24 5.375 18.635 0 12 0zm-1.788 15.307l-3.417-3.421v6.351H1.758C3.87 21.689 7.678 24 12 24s8.162-2.311 10.245-5.764h-5.04v-6.351l-3.386 3.421-1.788 1.79-1.814-1.79h-.005z"
        /> < title > { title } < / title > < / svg >
    }
}
