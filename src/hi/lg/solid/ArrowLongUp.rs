#[cfg(feature = "HiLgSolidArrowLongUp")]
use leptos::*;
#[cfg(feature = "HiLgSolidArrowLongUp")]
///This icon requires the feature `HiLgSolidArrowLongUp` to be enabled.
#[component]
pub fn ArrowLongUp(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M11.4697 2.46967C11.7626 2.17678 12.2374 2.17678 12.5303 2.46967L16.2803 6.21967C16.5732 6.51256 16.5732 6.98744 16.2803 7.28033C15.9874 7.57322 15.5126 7.57322 15.2197 7.28033L12.75 4.81066V21C12.75 21.4142 12.4142 21.75 12 21.75C11.5858 21.75 11.25 21.4142 11.25 21V4.81066L8.78033 7.28033C8.48744 7.57322 8.01256 7.57322 7.71967 7.28033C7.42678 6.98744 7.42678 6.51256 7.71967 6.21967L11.4697 2.46967Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
