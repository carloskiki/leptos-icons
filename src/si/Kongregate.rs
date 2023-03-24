#[cfg(feature = "SiKongregate")]
use leptos::*;
#[cfg(feature = "SiKongregate")]
///This icon requires the feature `SiKongregate` to be enabled.
#[component]
pub fn Kongregate(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M13.358 22.95v-3.736h1.551l.106-.141-3.877-5.851-3.172 3.264-.026 2.351.166.095 2.22 1.342.071 2.676H.141l.053-3.021 2.027-.715.106-.141V5.187l-.07-.141L0 4.165V.922h10.362v3.736h-2.22l-.106.141v7.014l7.472-6.802V4.87l-1.163-.352-1.163-.352V.922h10.75v3.736h-1.41l-.352.106-7.219 6.165 6.493 8.46.246.246 2.31.787v2.656l-10.642-.128z"
        /> < title > { title } < / title > < / svg >
    }
}
