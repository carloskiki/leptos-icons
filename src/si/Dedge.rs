#[cfg(feature = "SiDedge")]
use leptos::*;
#[cfg(feature = "SiDedge")]
///This icon requires the feature `SiDedge` to be enabled.
#[component]
pub fn Dedge(
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
        "M19.986 0v8.338C16.09 2.93 7.61 2.8 3.74 8.733-.523 15.27 4.191 23.99 11.996 24h.001c5.447-.003 9.872-4.43 9.87-9.877V0Zm-7.99 6.14a8.004 8.004 0 0 1 7.99 7.988 7.986 7.986 0 0 1-4.93 7.381 7.986 7.986 0 0 1-8.707-1.73 7.985 7.985 0 0 1-1.733-8.707 7.986 7.986 0 0 1 7.38-4.932Z"
        /> < title > { title } < / title > < / svg >
    }
}
