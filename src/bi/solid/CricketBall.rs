#[cfg(feature = "BiSolidCricketBall")]
use leptos::*;
#[cfg(feature = "BiSolidCricketBall")]
///This icon requires the feature `BiSolidCricketBall` to be enabled.
#[component]
pub fn CricketBall(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "m3.67 16.26.54.53-.62.61a9 9 0 0 0 .84 1.11L18.51 4.42a10.93 10.93 0 0 0-1.1-.83l-.62.61-.53-.53.48-.48A10 10 0 0 0 3.2 16.74zM14.86 5.07l.53.53L14 7l-.53-.53zm-2.79 2.8.52.53-1.39 1.4-.53-.53zm-2.8 2.8.53.53-1.4 1.39-.53-.53zm-2.8 2.79L7 14l-1.4 1.4-.53-.53zm.12 6.95.62-.61.53.53-.48.48A10 10 0 0 0 20.8 7.26l-.47.48-.54-.53.62-.61a9 9 0 0 0-.84-1.11L5.49 19.58a10.93 10.93 0 0 0 1.1.83zM18.4 8.61l.53.53-1.4 1.4L17 10zm-2.8 2.8.53.53-1.4 1.39-.53-.53zm-2.8 2.79.53.53-1.39 1.4-.54-.53zM10 17l.53.53-1.4 1.4-.53-.53z"
        /> < title > { title } < / title > < / svg >
    }
}
