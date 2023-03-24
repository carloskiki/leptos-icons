#[cfg(feature = "RiBusinessLineCreativeCommonsZero")]
use leptos::*;
#[cfg(feature = "RiBusinessLineCreativeCommonsZero")]
///This icon requires the feature `RiBusinessLineCreativeCommonsZero` to be enabled.
#[component]
pub fn CreativeCommonsZero(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M12 2c5.52 0 10 4.48 10 10s-4.48 10-10 10S2 17.52 2 12 6.48 2 12 2zm0 2c-4.415 0-8 3.585-8 8s3.585 8 8 8 8-3.585 8-8-3.585-8-8-8zm0 2c2.761 0 5 2.686 5 6s-2.239 6-5 6-5-2.686-5-6 2.239-6 5-6zm2.325 3.472l-3.562 6.173c.377.228.796.355 1.237.355 1.657 0 3-1.79 3-4 0-.959-.253-1.839-.675-2.528zM12 8c-1.657 0-3 1.79-3 4 0 .959.253 1.839.675 2.528l3.562-6.173A2.377 2.377 0 0 0 12 8z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
