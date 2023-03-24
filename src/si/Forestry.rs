#[cfg(feature = "SiForestry")]
use leptos::*;
#[cfg(feature = "SiForestry")]
///This icon requires the feature `SiForestry` to be enabled.
#[component]
pub fn Forestry(
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
        "M5.564.332v2.82L0 8.736l1.305 1.284 4.26-4.26v2.568L0 13.912l1.305 1.283 4.26-4.26v12.733h1.831V10.932l4.284 4.263 1.304-1.283-5.588-5.588V5.756l3.989 3.969 5.195 5.214v8.729h1.832v-8.725L24 9.355l-1.305-1.283-4.283 4.264V9.768L24 4.18l-1.305-1.284-4.283 4.264V.332H16.58v6.824l-4.26-4.26-1.304 1.284 5.564 5.584v2.568l-3.596-3.596-5.588-5.588V.332H5.564z"
        /> < title > { title } < / title > < / svg >
    }
}
