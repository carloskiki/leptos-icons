#[cfg(feature = "SiCoderwall")]
use leptos::*;
#[cfg(feature = "SiCoderwall")]
///This icon requires the feature `SiCoderwall` to be enabled.
#[component]
pub fn Coderwall(
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
        "M21.354 18.708c1.46 0 2.646 1.185 2.646 2.646C24 22.814 22.814 24 21.354 24s-2.646-1.186-2.646-2.646c0-1.458 1.185-2.646 2.646-2.646zM12 9.354c1.46 0 2.646 1.186 2.646 2.646S13.46 14.646 12 14.646 9.354 13.46 9.354 12 10.54 9.354 12 9.354zm9.354 0C22.814 9.354 24 10.54 24 12s-1.186 2.646-2.646 2.646S18.708 13.46 18.708 12s1.185-2.646 2.646-2.646zM12 0c1.46 0 2.646 1.185 2.646 2.646 0 1.46-1.186 2.646-2.646 2.646S9.354 4.106 9.354 2.646 10.54 0 12 0zM2.646 0c1.46 0 2.646 1.185 2.646 2.646 0 1.46-1.186 2.646-2.646 2.646S0 4.106 0 2.646 1.186 0 2.646 0zm18.708 0C22.814 0 24 1.185 24 2.646c0 1.46-1.186 2.646-2.646 2.646s-2.646-1.186-2.646-2.646S19.893 0 21.354 0z"
        /> < title > { title } < / title > < / svg >
    }
}
