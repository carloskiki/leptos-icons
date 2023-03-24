#[cfg(feature = "RiLogosFillMessenger")]
use leptos::*;
#[cfg(feature = "RiLogosFillMessenger")]
///This icon requires the feature `RiLogosFillMessenger` to be enabled.
#[component]
pub fn Messenger(
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
        "M12 2c5.634 0 10 4.127 10 9.7 0 5.573-4.366 9.7-10 9.7a10.894 10.894 0 0 1-2.895-.384.8.8 0 0 0-.534.039l-1.984.876a.8.8 0 0 1-1.123-.707l-.055-1.78a.797.797 0 0 0-.268-.57C3.195 17.135 2 14.617 2 11.7 2 6.127 6.367 2 12 2zM5.995 14.537c-.282.447.268.951.689.631l3.155-2.394a.6.6 0 0 1 .723 0l2.337 1.75a1.5 1.5 0 0 0 2.169-.4l2.937-4.66c.282-.448-.268-.952-.689-.633l-3.155 2.396a.6.6 0 0 1-.723 0l-2.337-1.75a1.5 1.5 0 0 0-2.169.4l-2.937 4.66z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
