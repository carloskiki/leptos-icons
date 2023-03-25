#[cfg(feature = "CgArrowLongLeftL")]
use leptos::*;
#[cfg(feature = "CgArrowLongLeftL")]
///This icon requires the feature `CgArrowLongLeftL` to be enabled.
#[component]
pub fn ArrowLongLeftL(
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
        "M5.20837 7.75725L0.969116 12.0033L5.21515 16.2428L6.62823 14.8274L4.80949 13.0116L21.0229 13.0298L21.0189 15.0297L23.0189 15.0338L23.0309 9.03377L21.0309 9.02976L21.0249 12.019L21.0261 11.0298L4.78543 11.0115L6.62371 9.17033L5.20837 7.75725Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
