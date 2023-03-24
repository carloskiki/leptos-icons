#[cfg(feature = "FaSolidMobileButton")]
use leptos::*;
#[cfg(feature = "FaSolidMobileButton")]
///This icon requires the feature `FaSolidMobileButton` to be enabled.
#[component]
pub fn MobileButton(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M64 0C28.7 0 0 28.7 0 64V448c0 35.3 28.7 64 64 64H288c35.3 0 64-28.7 64-64V64c0-35.3-28.7-64-64-64H64zM176 400a32 32 0 1 1 0 64 32 32 0 1 1 0-64z"
        /> < title > { title } < / title > < / svg >
    }
}
