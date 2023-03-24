#[cfg(feature = "BiLogosXing")]
use leptos::*;
#[cfg(feature = "BiLogosXing")]
///This icon requires the feature `BiLogosXing` to be enabled.
#[component]
pub fn Xing(
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
        "M5 6a.4.4 0 0 0-.38.62l2 3.38-3.06 5.4a.45.45 0 0 0 0 .45.43.43 0 0 0 .38.19h2.87a.89.89 0 0 0 .79-.55s3-5.31 3.11-5.51l-2-3.46A.91.91 0 0 0 7.92 6zm12.16-4a.84.84 0 0 0-.77.55L10 13.93l4.09 7.52a.91.91 0 0 0 .81.55h2.88a.43.43 0 0 0 .38-.18.45.45 0 0 0 0-.45l-4.07-7.43 6.36-11.31a.45.45 0 0 0 0-.45.44.44 0 0 0-.38-.18z"
        /> < title > { title } < / title > < / svg >
    }
}
