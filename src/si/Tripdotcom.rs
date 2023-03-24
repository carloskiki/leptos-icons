#[cfg(feature = "SiTripdotcom")]
use leptos::*;
#[cfg(feature = "SiTripdotcom")]
///This icon requires the feature `SiTripdotcom` to be enabled.
#[component]
pub fn Tripdotcom(
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
        "M17.834 9.002c-.68 0-1.29.31-1.707.799v-.514h-1.708v8.348h1.897v-2.923c.416.344.943.551 1.518.551 1.677 0 3.036-1.401 3.036-3.13s-1.36-3.13-3.036-3.13zm-.19 4.516c-.733 0-1.328-.62-1.328-1.385s.595-1.385 1.328-1.385c.734 0 1.328.62 1.328 1.385s-.594 1.385-1.328 1.385zm6.356.607a1.138 1.138 0 1 1-2.277 0 1.138 1.138 0 0 1 2.277 0zM13.205 7.428a1.062 1.062 0 1 1-2.125 0 1.062 1.062 0 0 1 2.125 0zm-2.011 1.859h1.897v5.692h-1.897V9.287zM6.83 8.225H4.364v6.754H2.466V8.225H0V6.63h6.83v1.594zm3.035 1.033c.13 0 .255.012.38.03v1.74a1.55 1.55 0 0 0-.297-.031c-.88 0-1.594.612-1.594 1.593v2.389H6.451V9.287h1.707v.9c.363-.558.991-.93 1.707-.93z"
        /> < title > { title } < / title > < / svg >
    }
}
