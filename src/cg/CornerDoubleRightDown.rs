#[cfg(feature = "CgCornerDoubleRightDown")]
use leptos::*;
#[cfg(feature = "CgCornerDoubleRightDown")]
///This icon requires the feature `CgCornerDoubleRightDown` to be enabled.
#[component]
pub fn CornerDoubleRightDown(
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
        "M7.69438 12.7048L2.63776 7.86424L7.47827 2.80762L8.92302 4.19062L6.56623 6.65263L12.6002 6.49061C15.2502 6.41945 17.4561 8.51002 17.5273 11.16L17.6864 17.0862L19.9335 14.7924L21.3622 16.192L16.4636 21.1924L11.4632 16.2938L12.8628 14.8651L15.2897 17.2426L15.1281 11.2245C15.0926 9.89945 13.9896 8.85417 12.6646 8.88974L6.76674 9.04811L9.07738 11.26L7.69438 12.7048Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
