#[cfg(feature = "SiNdr")]
use leptos::*;
#[cfg(feature = "SiNdr")]
///This icon requires the feature `SiNdr` to be enabled.
#[component]
pub fn Ndr(
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
        "M5.184 19.325l-3.137-5.648v5.649H0V9.64h2.149l3.267 6.025V9.641h2.047v9.684zm2.279-9.684V.537H8.61v9.104zm0 13.822v-4.138H8.61v4.138zM12.037 9.64c2.395 0 3.63 1.147 3.63 3.368v2.918c0 2.28-1.19 3.398-3.63 3.398H8.61V9.641zm-.19 7.855c1.163 0 1.728-.581 1.728-1.771v-2.498c0-1.176-.58-1.757-1.727-1.757h-1.03v6.026zm9.845 1.83l-1.728-3.718h-1.161v3.717h-2.15V9.641h3.384c2.381 0 3.513.944 3.513 2.962 0 1.335-.493 2.134-1.597 2.613L24 19.326zm-1.568-5.475c.857 0 1.365-.494 1.365-1.32 0-.858-.377-1.177-1.365-1.177H18.76v2.498h1.365z"
        /> < title > { title } < / title > < / svg >
    }
}
