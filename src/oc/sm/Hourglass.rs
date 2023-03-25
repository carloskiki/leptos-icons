#[cfg(feature = "OcSmHourglass")]
use leptos::*;
#[cfg(feature = "OcSmHourglass")]
///This icon requires the feature `OcSmHourglass` to be enabled.
#[component]
pub fn Hourglass(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M2.75 1h10.5a.75.75 0 0 1 0 1.5h-.75v1.25a4.75 4.75 0 0 1-1.9 3.8l-.333.25a.25.25 0 0 0 0 .4l.333.25a4.75 4.75 0 0 1 1.9 3.8v1.25h.75a.75.75 0 0 1 0 1.5H2.75a.75.75 0 0 1 0-1.5h.75v-1.25a4.75 4.75 0 0 1 1.9-3.8l.333-.25a.25.25 0 0 0 0-.4L5.4 7.55a4.75 4.75 0 0 1-1.9-3.8V2.5h-.75a.75.75 0 0 1 0-1.5ZM11 2.5H5v1.25c0 1.023.482 1.986 1.3 2.6l.333.25c.934.7.934 2.1 0 2.8l-.333.25a3.251 3.251 0 0 0-1.3 2.6v1.25h6v-1.25a3.251 3.251 0 0 0-1.3-2.6l-.333-.25a1.748 1.748 0 0 1 0-2.8l.333-.25a3.251 3.251 0 0 0 1.3-2.6Z"
        /> < title > { title } < / title > < / svg >
    }
}
