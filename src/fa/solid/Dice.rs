#[cfg(feature = "FaSolidDice")]
use leptos::*;
#[cfg(feature = "FaSolidDice")]
///This icon requires the feature `FaSolidDice` to be enabled.
#[component]
pub fn Dice(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M252.3 11.7c-15.6-15.6-40.9-15.6-56.6 0l-184 184c-15.6 15.6-15.6 40.9 0 56.6l184 184c15.6 15.6 40.9 15.6 56.6 0l184-184c15.6-15.6 15.6-40.9 0-56.6l-184-184zM248 224c0 13.3-10.7 24-24 24s-24-10.7-24-24s10.7-24 24-24s24 10.7 24 24zM96 248c-13.3 0-24-10.7-24-24s10.7-24 24-24s24 10.7 24 24s-10.7 24-24 24zm128 80c13.3 0 24 10.7 24 24s-10.7 24-24 24s-24-10.7-24-24s10.7-24 24-24zm128-80c-13.3 0-24-10.7-24-24s10.7-24 24-24s24 10.7 24 24s-10.7 24-24 24zM224 72c13.3 0 24 10.7 24 24s-10.7 24-24 24s-24-10.7-24-24s10.7-24 24-24zm96 392c0 26.5 21.5 48 48 48H592c26.5 0 48-21.5 48-48V240c0-26.5-21.5-48-48-48H472.5c13.4 26.9 8.8 60.5-13.6 82.9L320 413.8V464zm160-88c-13.3 0-24-10.7-24-24s10.7-24 24-24s24 10.7 24 24s-10.7 24-24 24z"
        /> < title > { title } < / title > < / svg >
    }
}
