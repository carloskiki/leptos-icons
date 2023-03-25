#[cfg(feature = "SiAffinitypublisher")]
use leptos::*;
#[cfg(feature = "SiAffinitypublisher")]
///This icon requires the feature `SiAffinitypublisher` to be enabled.
#[component]
pub fn Affinitypublisher(
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
        "M10.44 0L9.36 1.87 22.136 24h1.144a.72.72 0 00.72-.72v-3.119L12.36 0zm3 0L24 18.29V.72a.72.72 0 00-.72-.72zM8.82 2.806l-1.98 3.43L16.976 24h4.08zM6.24 7.274L4.76 9.837a1.941 1.941 0 000 1.942C6.531 14.842 11.816 24 11.816 24h4.08zm-2.58 4.47L0 18.082v5.197c0 .397.323.72.72.72h10.016Z"
        /> < title > { title } < / title > < / svg >
    }
}
