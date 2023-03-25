#[cfg(feature = "CgArrowLongDownL")]
use leptos::*;
#[cfg(feature = "CgArrowLongDownL")]
///This icon requires the feature `CgArrowLongDownL` to be enabled.
#[component]
pub fn ArrowLongDownL(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.99801 0.972107V2.97211H11.9983L10.998 2.97299L11.0124 19.2135L9.16751 17.379L7.75726 18.7972L12.0118 23.0279L16.2427 18.7734L14.8246 17.3632L13.0124 19.1856L12.998 2.97211H14.998V0.972107H8.99801Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
