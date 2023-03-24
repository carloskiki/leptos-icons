#[cfg(feature = "SiApachesolr")]
use leptos::*;
#[cfg(feature = "SiApachesolr")]
///This icon requires the feature `SiApachesolr` to be enabled.
#[component]
pub fn Apachesolr(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M20.741 3.8L8.926 16.573l14.849-6.851A11.979 11.979 0 0 0 20.741 3.8M11.975 0c-1.637 0-3.197.328-4.619.921l-1.585 13.36L13.693.124A12.168 12.168 0 0 0 11.975 0m11.918 10.459l-14.07 7.874 13.201-1.566a11.976 11.976 0 0 0 .869-6.308m-5.188 11.527a12.084 12.084 0 0 0 3.8-4.16l-12.374 2.457 8.574 1.703zM14.417.249L7.53 15.177 20.306 3.36A11.978 11.978 0 0 0 14.417.249M12.98 24a11.938 11.938 0 0 0 3.774-.945l-6.931-.822L12.98 24zM1.016 7.08a11.944 11.944 0 0 0-1.013 3.864l1.867 3.337-.854-7.201zm5.298-5.665a12.076 12.076 0 0 0-4.236 3.784l1.743 8.773L6.314 1.415z"
        /> < title > { title } < / title > < / svg >
    }
}
