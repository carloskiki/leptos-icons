#[cfg(feature = "SiLudwig")]
use leptos::*;
#[cfg(feature = "SiLudwig")]
///This icon requires the feature `SiLudwig` to be enabled.
#[component]
pub fn Ludwig(
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
        "M0 0h24v24H0V0Zm13.714 10.286V3.429h-3.428v10.285H20.57V3.43h-3.428v6.857h-3.429Zm-6.857 6.857V3.429H3.43V20.57h17.14v-3.428H6.857Z"
        /> < title > { title } < / title > < / svg >
    }
}
