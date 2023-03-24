#[cfg(feature = "RiBusinessLineCreativeCommonsSa")]
use leptos::*;
#[cfg(feature = "RiBusinessLineCreativeCommonsSa")]
///This icon requires the feature `RiBusinessLineCreativeCommonsSa` to be enabled.
#[component]
pub fn CreativeCommonsSa(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path fill - rule = "nonzero" d =
        "M12 2c5.52 0 10 4.48 10 10s-4.48 10-10 10S2 17.52 2 12 6.48 2 12 2zm0 2c-4.415 0-8 3.585-8 8s3.585 8 8 8 8-3.585 8-8-3.585-8-8-8zm0 2c2.761 0 5 2.686 5 6s-2.239 6-5 6c-2.177 0-4.029-1.67-4.715-4l2.117.001C9.92 15.196 10.89 16 12 16c1.657 0 3-1.79 3-4s-1.343-4-3-4c-1.11 0-2.08.805-2.599 2H11l-2.5 3L6 10h1.284C7.971 7.67 9.823 6 12 6z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
