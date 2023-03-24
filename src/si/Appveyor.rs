#[cfg(feature = "SiAppveyor")]
use leptos::*;
#[cfg(feature = "SiAppveyor")]
///This icon requires the feature `SiAppveyor` to be enabled.
#[component]
pub fn Appveyor(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M 12,0 C 18.6,0 24,5.4 24,12 24,18.6 18.6,24 12,24 5.4,24 0,18.6 0,12 0,5.4 5.4,0 12,0 Z m 2.94,14.34 C 16.26,12.66 16.08,10.26 14.4,9 12.78,7.74 10.38,8.04 9,9.72 7.68,11.4 7.86,13.8 9.54,15.06 c 1.68,1.26 4.08,0.96 5.4,-0.72 z m -6.42,7.8 c 0.72,0.3 2.28,0.6 3.06,0.6 l 5.22,-7.56 c 1.68,-2.52 1.26,-5.94 -1.08,-7.8 -2.1,-1.68 -5.04,-1.62 -7.14,0 l -7.26,5.58 c 0.18,1.92 0.72,2.88 0.72,2.94 l 4.14,-4.5 c -0.3,1.98 0.42,4.02 2.1,5.28 1.44,1.14 3.18,1.44 4.86,1.08 z"
        /> < title > { title } < / title > < / svg >
    }
}
