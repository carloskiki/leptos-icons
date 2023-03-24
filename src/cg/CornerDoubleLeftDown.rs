#[cfg(feature = "CgCornerDoubleLeftDown")]
use leptos::*;
#[cfg(feature = "CgCornerDoubleLeftDown")]
///This icon requires the feature `CgCornerDoubleLeftDown` to be enabled.
#[component]
pub fn CornerDoubleLeftDown(
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
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M21.299 7.7609L16.2801 12.6405L14.8859 11.2065L17.3217 8.83829L11.3015 8.85324C9.97605 8.85653 8.9042 9.93371 8.9075 11.2592L8.92214 17.1591L11.1898 14.903L12.6004 16.3208L7.63814 21.258L2.70093 16.2957L4.11871 14.8851L6.52249 17.3011L6.5075 11.2652C6.50092 8.61419 8.64461 6.45983 11.2956 6.45325L17.224 6.43853L14.9855 4.1361L16.4195 2.74194L21.299 7.7609Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
