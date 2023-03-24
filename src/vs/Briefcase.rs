#[cfg(feature = "VsBriefcase")]
use leptos::*;
#[cfg(feature = "VsBriefcase")]
///This icon requires the feature `VsBriefcase` to be enabled.
#[component]
pub fn Briefcase(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M14.5 4H11V2.5l-.5-.5h-5l-.5.5V4H1.5l-.5.5v8l.5.5h13l.5-.5v-8l-.5-.5zM6 3h4v1H6V3zm8 2v.76L10 8v-.5L9.51 7h-3L6 7.5V8L2 5.71V5h12zM9 8v1H7V8h2zm-7 4V6.86l4 2.29v.35l.5.5h3l.5-.5v-.31l4-2.28V12H2z"
        /> < title > { title } < / title > < / svg >
    }
}
