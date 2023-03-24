#[cfg(feature = "CgArrowLongUpL")]
use leptos::*;
#[cfg(feature = "CgArrowLongUpL")]
///This icon requires the feature `CgArrowLongUpL` to be enabled.
#[component]
pub fn ArrowLongUpL(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.0321 1.01865L16.2425 5.29327L14.8177 6.69677L13.0192 4.87094L12.9676 20.9813H14.9644V22.9813H8.96441V20.9813H10.9676L11.0194 4.82354L9.16107 6.65399L7.75757 5.22914L12.0321 1.01865Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
