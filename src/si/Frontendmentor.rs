#[cfg(feature = "SiFrontendmentor")]
use leptos::*;
#[cfg(feature = "SiFrontendmentor")]
///This icon requires the feature `SiFrontendmentor` to be enabled.
#[component]
pub fn Frontendmentor(
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
        "M12.1706 1.2719a.732.732 0 00-.7186.732v13.914a.732.732 0 00.732.732.732.732 0 00.7318-.732V2.004a.732.732 0 00-.7452-.732zm11.0741 4.1685a.7339.7339 0 00-.2764.063L16.686 8.307a.7329.7329 0 000 1.3361l6.2823 2.8134a.7378.7378 0 00.2993.0648.732.732 0 00.2973-1.401l-4.786-2.1443 4.786-2.1366a.7339.7339 0 00.3698-.9664.7339.7339 0 00-.69-.4327zm-22.499 5.032a.7316.7316 0 00-.7223.9149c1.736 6.677 7.7748 11.341 14.6822 11.341a.732.732 0 000-1.464 13.7055 13.7055 0 01-13.266-10.2449.7316.7316 0 00-.6939-.547z"
        /> < title > { title } < / title > < / svg >
    }
}
