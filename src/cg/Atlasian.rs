#[cfg(feature = "CgAtlasian")]
use leptos::*;
#[cfg(feature = "CgAtlasian")]
///This icon requires the feature `CgAtlasian` to be enabled.
#[component]
pub fn Atlasian(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg"
        opacity = "0.8" d =
        "M8.50705 11.5562C8.19028 11.1038 7.78219 11.1585 7.59556 11.6783L5 18.9075H10.1778C10.6982 16.85 10.2481 14.0427 9.08063 12.3754L8.50705 11.5562Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.8745 7.12641C11.6074 8.93603 11.1992 12.0835 11.9627 14.1565L13.7126 18.9074H18.9644L14.3673 6.42648C14.1764 5.90823 13.7649 5.85485 13.4481 6.30726L12.8745 7.12641Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
