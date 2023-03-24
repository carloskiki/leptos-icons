#[cfg(feature = "RiBusinessFillCreativeCommonsSa")]
use leptos::*;
#[cfg(feature = "RiBusinessFillCreativeCommonsSa")]
///This icon requires the feature `RiBusinessFillCreativeCommonsSa` to be enabled.
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
        "none" d = "M0 0h24v24H0z" />< path d =
        "M12 2c5.52 0 10 4.48 10 10s-4.48 10-10 10S2 17.52 2 12 6.48 2 12 2zm0 4C9.895 6 8.094 7.56 7.357 9.77l-.073.23H6l2.5 3 2.5-3H9.401C9.92 8.805 10.89 8 12 8c1.657 0 3 1.79 3 4s-1.343 4-3 4c-1.048 0-1.971-.717-2.508-1.803L9.402 14H7.285C7.97 16.33 9.823 18 12 18c2.761 0 5-2.686 5-6s-2.239-6-5-6z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
