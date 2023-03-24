#[cfg(feature = "FaSolidHillAvalanche")]
use leptos::*;
#[cfg(feature = "FaSolidHillAvalanche")]
///This icon requires the feature `FaSolidHillAvalanche` to be enabled.
#[component]
pub fn HillAvalanche(
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
        "M440.7 401.9c34.2 23.1 81.1 19.5 111.4-10.8c34.4-34.4 34.4-90.1 0-124.4c-27.8-27.8-69.5-33.1-102.6-16c-11.8 6.1-16.4 20.6-10.3 32.3s20.6 16.4 32.3 10.3c15.1-7.8 34-5.3 46.6 7.3c15.6 15.6 15.6 40.9 0 56.6s-40.9 15.6-56.6 0l-81.7-81.7C402.2 261.3 417 236.4 417 208c0-33.9-21.1-62.9-50.9-74.5c1.9-6.8 2.9-14 2.9-21.5c0-44.2-35.8-80-80-80c-27.3 0-51.5 13.7-65.9 34.6C217.3 46.6 198.9 32 177 32c-26.5 0-48 21.5-48 48c0 4 .5 7.9 1.4 11.6L440.7 401.9zM481 64a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm0 128a32 32 0 1 0 0-64 32 32 0 1 0 0 64zM69.3 87C44.1 61.8 1 79.7 1 115.3V432c0 44.2 35.8 80 80 80H397.7c35.6 0 53.5-43.1 28.3-68.3L69.3 87z"
        /> < title > { title } < / title > < / svg >
    }
}
