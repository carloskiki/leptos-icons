#[cfg(feature = "OcLgUnlink")]
use leptos::*;
#[cfg(feature = "OcLgUnlink")]
///This icon requires the feature `OcLgUnlink` to be enabled.
#[component]
pub fn Unlink(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M20.347 3.653a3.936 3.936 0 0 0-5.567 0l-1.75 1.75a.75.75 0 0 1-1.06-1.06l1.75-1.75a5.436 5.436 0 0 1 7.688 7.687l-1.564 1.564a.75.75 0 0 1-1.06-1.06l1.563-1.564a3.936 3.936 0 0 0 0-5.567ZM9.786 12.369a.75.75 0 0 1 1.053.125c.096.122.2.24.314.353 1.348 1.348 3.386 1.587 4.89.658l-3.922-2.858a.745.745 0 0 1-.057-.037c-1.419-1.013-3.454-.787-4.784.543L3.653 14.78a3.936 3.936 0 0 0 5.567 5.567l3-3a.75.75 0 1 1 1.06 1.06l-3 3a5.436 5.436 0 1 1-7.688-7.687l3.628-3.628a5.517 5.517 0 0 1 3.014-1.547l-7.05-5.136a.75.75 0 0 1 .883-1.213l20.25 14.75a.75.75 0 0 1-.884 1.213l-5.109-3.722c-2.155 1.709-5.278 1.425-7.232-.53a5.491 5.491 0 0 1-.431-.485.75.75 0 0 1 .125-1.053Z"
        /> < title > { title } < / title > < / svg >
    }
}
