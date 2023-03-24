#[cfg(feature = "SiBeats")]
use leptos::*;
#[cfg(feature = "SiBeats")]
///This icon requires the feature `SiBeats` to be enabled.
#[component]
pub fn Beats(
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
        "M2.625 0v15h8.25a7.5 7.5 0 0 0 0-15zm17.016 11.705c-1.571 3.261-4.91 5.517-8.766 5.517h-8.25V24h11.25a7.5 7.5 0 0 0 5.766-12.295z"
        /> < title > { title } < / title > < / svg >
    }
}
