#[cfg(feature = "RiOthersFillScales2")]
use leptos::*;
#[cfg(feature = "RiOthersFillScales2")]
///This icon requires the feature `RiOthersFillScales2` to be enabled.
#[component]
pub fn Scales2(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M6 2c0 .513.49 1 1 1h10c.513 0 1-.49 1-1h2c0 1.657-1.343 3-3 3h-4l.001 2.062C16.947 7.555 20 10.921 20 15v6c0 .552-.448 1-1 1H5c-.552 0-1-.448-1-1v-6c0-4.08 3.054-7.446 7-7.938V5H7C5.34 5 4 3.66 4 2h2zm6 9c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4c0-.742-.202-1.436-.554-2.032l-2.739 2.74-.094.082c-.392.305-.96.278-1.32-.083-.39-.39-.39-1.024 0-1.414l2.739-2.74C13.436 11.203 12.742 11 12 11z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
