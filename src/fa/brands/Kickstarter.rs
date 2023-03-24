#[cfg(feature = "FaBrandsKickstarter")]
use leptos::*;
#[cfg(feature = "FaBrandsKickstarter")]
///This icon requires the feature `FaBrandsKickstarter` to be enabled.
#[component]
pub fn Kickstarter(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M400 480H48c-26.4 0-48-21.6-48-48V80c0-26.4 21.6-48 48-48h352c26.4 0 48 21.6 48 48v352c0 26.4-21.6 48-48 48zM199.6 178.5c0-30.7-17.6-45.1-39.7-45.1-25.8 0-40 19.8-40 44.5v154.8c0 25.8 13.7 45.6 40.5 45.6 21.5 0 39.2-14 39.2-45.6v-41.8l60.6 75.7c12.3 14.9 39 16.8 55.8 0 14.6-15.1 14.8-36.8 4-50.4l-49.1-62.8 40.5-58.7c9.4-13.5 9.5-34.5-5.6-49.1-16.4-15.9-44.6-17.3-61.4 7l-44.8 64.7v-38.8z"
        /> < title > { title } < / title > < / svg >
    }
}
