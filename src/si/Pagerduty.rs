#[cfg(feature = "SiPagerduty")]
use leptos::*;
#[cfg(feature = "SiPagerduty")]
///This icon requires the feature `SiPagerduty` to be enabled.
#[component]
pub fn Pagerduty(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M16.965 1.18C15.085.164 13.769 0 10.683 0H3.73v14.55h6.926c2.743 0 4.8-.164 6.61-1.37 1.975-1.303 3.004-3.484 3.004-6.007 0-2.716-1.262-4.896-3.305-5.994zm-5.5 10.326h-4.21V3.113l3.977-.027c3.62-.028 5.43 1.234 5.43 4.128 0 3.113-2.248 4.292-5.197 4.292zM3.73 17.61h3.525V24H3.73Z"
        /> < title > { title } < / title > < / svg >
    }
}
