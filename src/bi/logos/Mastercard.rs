#[cfg(feature = "BiLogosMastercard")]
use leptos::*;
#[cfg(feature = "BiLogosMastercard")]
///This icon requires the feature `BiLogosMastercard` to be enabled.
#[component]
pub fn Mastercard(
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
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M11.454 17.021c.048.041.1.082.151.122a6.173 6.173 0 0 1-3.42 1.03A6.17 6.17 0 0 1 2.01 12a6.175 6.175 0 0 1 9.592-5.144c-.05.043-.1.082-.138.126A6.633 6.633 0 0 0 9.166 12c0 1.925.833 3.755 2.288 5.021zm4.361-11.195a6.14 6.14 0 0 0-3.416 1.03c.049.043.099.082.137.126 1.462 1.263 2.299 3.094 2.299 5.018s-.835 3.753-2.288 5.021c-.049.041-.101.082-.151.122a6.162 6.162 0 0 0 3.418 1.03 6.174 6.174 0 1 0 .001-12.347zM12 7.15A6.152 6.152 0 0 0 9.644 12 6.15 6.15 0 0 0 12 16.853 6.157 6.157 0 0 0 14.357 12 6.15 6.15 0 0 0 12 7.15z"
        /> < title > { title } < / title > < / svg >
    }
}
