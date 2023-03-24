#[cfg(feature = "SiBitdefender")]
use leptos::*;
#[cfg(feature = "SiBitdefender")]
///This icon requires the feature `SiBitdefender` to be enabled.
#[component]
pub fn Bitdefender(
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
        "M1.685 0v.357l1.232 1.046c1.477 1.204 1.67 1.439 1.67 2.526V24h8.646c4.537 0 9.083-1.629 9.083-6.849 0-3.082-2.174-5.458-5.186-5.797v-.067c2.475-.745 4.169-2.54 4.169-5.253 0-4.372-3.73-6.032-7.349-6.032L1.686 0zm7.176 3.664h3.524c2.383 0 3.121.327 3.844 1.013.548.521.799 1.237.801 2.07 0 .775-.267 1.466-.831 2.004-.705.676-1.674 1.011-3.443 1.011H8.862V3.664zm0 9.758h4.099c3.456 0 5.085.881 5.085 3.39 0 3.153-3.055 3.526-5.256 3.526H8.86v-6.916z"
        /> < title > { title } < / title > < / svg >
    }
}
