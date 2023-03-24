#[cfg(feature = "RiOthersFillScales3")]
use leptos::*;
#[cfg(feature = "RiOthersFillScales3")]
///This icon requires the feature `RiOthersFillScales3` to be enabled.
#[component]
pub fn Scales3(
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
        "none" d = "M0 0H24V24H0z" />< path d =
        "M13 2v1.278l5 1.668 3.632-1.21.633 1.896-3.032 1.011 3.096 8.512C21.237 16.292 19.7 17 18 17c-1.701 0-3.237-.708-4.329-1.845l3.094-8.512L13 5.387V19H17v2H7v-2h4V5.387L7.232 6.643l3.096 8.512C9.237 16.292 7.7 17 6 17c-1.701 0-3.237-.708-4.329-1.845l3.094-8.512-3.03-1.01.633-1.898L6 4.945l5-1.667V2h2zm5 7.103L16.582 13h2.835L18 9.103zm-12 0L4.582 13h2.835L6 9.103z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
