#[cfg(feature = "BiRegularCarousel")]
use leptos::*;
#[cfg(feature = "BiRegularCarousel")]
///This icon requires the feature `BiRegularCarousel` to be enabled.
#[component]
pub fn Carousel(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M4 19h2c0 1.103.897 2 2 2h8c1.103 0 2-.897 2-2h2c1.103 0 2-.897 2-2V7c0-1.103-.897-2-2-2h-2c0-1.103-.897-2-2-2H8c-1.103 0-2 .897-2 2H4c-1.103 0-2 .897-2 2v10c0 1.103.897 2 2 2zM20 7v10h-2V7h2zM8 5h8l.001 14H8V5zM4 7h2v10H4V7z"
        /> < title > { title } < / title > < / svg >
    }
}
