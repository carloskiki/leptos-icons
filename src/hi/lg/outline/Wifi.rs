#[cfg(feature = "HiLgOutlineWifi")]
use leptos::*;
#[cfg(feature = "HiLgOutlineWifi")]
///This icon requires the feature `HiLgOutlineWifi` to be enabled.
#[component]
pub fn Wifi(
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
        "M8.28767 15.0378C10.3379 12.9875 13.662 12.9875 15.7123 15.0378M5.10569 11.8558C8.9133 8.04815 15.0867 8.04815 18.8943 11.8558M1.92371 8.67373C7.48868 3.10876 16.5113 3.10876 22.0762 8.67373M12.5303 18.2197L12 18.7501L11.4696 18.2197C11.7625 17.9268 12.2374 17.9268 12.5303 18.2197Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
