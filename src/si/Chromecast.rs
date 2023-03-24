#[cfg(feature = "SiChromecast")]
use leptos::*;
#[cfg(feature = "SiChromecast")]
///This icon requires the feature `SiChromecast` to be enabled.
#[component]
pub fn Chromecast(
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
        "M0 18.5455v3.2727h3.2727c0-1.811-1.4618-3.2727-3.2727-3.2727zm0-4.3637v2.1818c3.011 0 5.4545 2.4437 5.4545 5.4546h2.1819c0-4.2218-3.4146-7.6364-7.6364-7.6364zm0-4.3636V12c5.4218 0 9.8182 4.3964 9.8182 9.8182H12c0-6.6327-5.3782-12-12-12zm21.8182-7.6364H2.1818C.9818 2.1818 0 3.1636 0 4.3636v3.2728h2.1818V4.3636h19.6364v15.2728h-7.6364v2.1818h7.6364c1.2 0 2.1818-.9818 2.1818-2.1818V4.3636c0-1.2-.9818-2.1818-2.1818-2.1818Z"
        /> < title > { title } < / title > < / svg >
    }
}
