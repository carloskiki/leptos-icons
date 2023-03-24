#[cfg(feature = "FaSolidVest")]
use leptos::*;
#[cfg(feature = "FaSolidVest")]
///This icon requires the feature `FaSolidVest` to be enabled.
#[component]
pub fn Vest(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M207.1 237.4L151.2 69.7C168.6 79.7 192.6 88 224 88s55.4-8.3 72.8-18.3L226.5 280.6c-1.6 4.9-2.5 10-2.5 15.2V464c0 26.5 21.5 48 48 48H400c26.5 0 48-21.5 48-48V270.5c0-9.5-2.8-18.7-8.1-26.6l-47.9-71.8c-5.3-7.9-8.1-17.1-8.1-26.6V128 54.3 48c0-26.5-21.5-48-48-48h-4.5c-.2 0-.4 0-.6 0c-.4 0-.8 0-1.2 0C311 0 295.7 9.7 285.7 18.8C276.4 27.2 257.2 40 224 40s-52.4-12.8-61.7-21.2C152.3 9.7 137 0 118.3 0c-.4 0-.8 0-1.2 0c-.2 0-.4 0-.6 0H112C85.5 0 64 21.5 64 48v6.3V128v17.5c0 9.5-2.8 18.7-8.1 26.6L8.1 243.9C2.8 251.8 0 261.1 0 270.5V464c0 26.5 21.5 48 48 48H176c9.9 0 19-3 26.7-8.1C195.9 492.2 192 478.5 192 464V295.8c0-8.6 1.4-17.1 4.1-25.3l11-33.1zM347.3 356.7l48 48c6.2 6.2 6.2 16.4 0 22.6s-16.4 6.2-22.6 0l-48-48c-6.2-6.2-6.2-16.4 0-22.6s16.4-6.2 22.6 0zm-294.6 48l48-48c6.2-6.2 16.4-6.2 22.6 0s6.2 16.4 0 22.6l-48 48c-6.2 6.2-16.4 6.2-22.6 0s-6.2-16.4 0-22.6z"
        /> < title > { title } < / title > < / svg >
    }
}
