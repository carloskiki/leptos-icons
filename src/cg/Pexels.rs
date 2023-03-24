#[cfg(feature = "CgPexels")]
use leptos::*;
#[cfg(feature = "CgPexels")]
///This icon requires the feature `CgPexels` to be enabled.
#[component]
pub fn Pexels(
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
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M12 5C12.7111 5 13.3875 5.14845 14 5.41604C15.7659 6.1876 17 7.94968 17 10C17 12.0503 15.7659 13.8124 14 14.584C13.3875 14.8516 12.7111 15 12 15V19H6V5H12ZM8 7V17H10V13H12L12.0032 12.9988C13.6427 13.0303 15.0746 11.6934 15.0443 9.95469L15.0375 9.56529C15.0121 8.10183 13.7882 6.94549 12.3257 7.00299L12.0203 7.00762L12 7H8Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
