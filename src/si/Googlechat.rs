#[cfg(feature = "SiGooglechat")]
use leptos::*;
#[cfg(feature = "SiGooglechat")]
///This icon requires the feature `SiGooglechat` to be enabled.
#[component]
pub fn Googlechat(
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
        "M7.533 0a1.816 1.816 0 0 0-1.816 1.816v2.832h11.178c1.043 0 1.888.855 1.888 1.91v8.204h2.906a1.816 1.816 0 0 0 1.817-1.817V1.816A1.816 1.816 0 0 0 21.689 0H7.533zM2.311 5.148A1.816 1.816 0 0 0 .494 6.965V23.09c0 .81.979 1.215 1.55.642l3.749-3.748h10.674a1.816 1.816 0 0 0 1.816-1.816V6.965a1.816 1.816 0 0 0-1.816-1.817H2.31Z"
        /> < title > { title } < / title > < / svg >
    }
}
