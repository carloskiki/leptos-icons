#[cfg(feature = "SiNorco")]
use leptos::*;
#[cfg(feature = "SiNorco")]
///This icon requires the feature `SiNorco` to be enabled.
#[component]
pub fn Norco(
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
        "M11.055 2.707a.971.971 0 00-.688.387L0 16.78h4.049l7.27-9.597 1.927 5.74 1.42-1.875-2.578-7.676a.983.983 0 00-1.033-.666zM19.95 7.22l-7.27 9.597-1.927-5.74-1.42 1.875 2.578 7.676a.987.987 0 001.72.28L24 7.218z"
        /> < title > { title } < / title > < / svg >
    }
}
