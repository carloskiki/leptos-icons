#[cfg(feature = "FaSolidScrollTorah")]
use leptos::*;
#[cfg(feature = "FaSolidScrollTorah")]
///This icon requires the feature `FaSolidScrollTorah` to be enabled.
#[component]
pub fn ScrollTorah(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M96 480V32C96 14.3 74.5 0 48 0S0 14.3 0 32V480c0 17.7 21.5 32 48 32s48-14.3 48-32zM512 32H128V480H512V32zM592 0c-26.5 0-48 14.3-48 32V480c0 17.7 21.5 32 48 32s48-14.3 48-32V32c0-17.7-21.5-32-48-32zM196 313.7c0-3.2 .9-6.4 2.5-9.2L226.7 256l-28.3-48.5c-1.6-2.8-2.5-6-2.5-9.2c0-10.1 8.2-18.3 18.3-18.3H271l31.4-53.9c3.6-6.3 10.3-10.1 17.6-10.1s13.9 3.8 17.6 10.1L369 180h56.7c10.1 0 18.3 8.2 18.3 18.3c0 3.2-.9 6.4-2.5 9.2L413.3 256l28.3 48.5c1.6 2.8 2.5 6 2.5 9.2c0 10.1-8.2 18.3-18.3 18.3H369l-31.4 53.9c-3.6 6.3-10.3 10.1-17.6 10.1s-13.9-3.8-17.6-10.1L271 332H214.3c-10.1 0-18.3-8.2-18.3-18.3zm124 54.7L341.2 332H298.8L320 368.4zM254.5 256l30.3 52h70.4l30.3-52-30.3-52H284.8l-30.3 52zm144.9 23.8L383 308h32.8l-16.4-28.2zM415.8 204H383l16.4 28.2L415.8 204zM320 143.6L298.8 180h42.4L320 143.6zM224.2 204l16.4 28.2L257 204H224.2zM257 308l-16.4-28.2L224.2 308H257z"
        /> < title > { title } < / title > < / svg >
    }
}
