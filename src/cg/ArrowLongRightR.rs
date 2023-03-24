#[cfg(feature = "CgArrowLongRightR")]
use leptos::*;
#[cfg(feature = "CgArrowLongRightR")]
///This icon requires the feature `CgArrowLongRightR` to be enabled.
#[component]
pub fn ArrowLongRightR(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M19.3514 7.72525L23.6012 11.9607L19.3655 16.2105L17.9489 14.7987L19.7633 12.9781L7.89744 13.0188L4.64151 16.2748L0.398865 12.0321L4.64151 7.78947L7.87093 11.0189L19.782 10.9781L17.9395 9.14185L19.3514 7.72525ZM4.64151 13.4463L6.05572 12.0321L4.64151 10.6179L3.22729 12.0321L4.64151 13.4463Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
