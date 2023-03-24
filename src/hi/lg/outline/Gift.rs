#[cfg(feature = "HiLgOutlineGift")]
use leptos::*;
#[cfg(feature = "HiLgOutlineGift")]
///This icon requires the feature `HiLgOutlineGift` to be enabled.
#[component]
pub fn Gift(
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
        "M21 11.25V19.5C21 20.3284 20.3284 21 19.5 21H5.25C4.42157 21 3.75 20.3284 3.75 19.5V11.25M12 4.875C12 3.42525 10.8247 2.25 9.375 2.25C7.92525 2.25 6.75 3.42525 6.75 4.875C6.75 6.32475 7.92525 7.5 9.375 7.5C10.1095 7.5 12 7.5 12 7.5M12 4.875C12 5.59024 12 7.5 12 7.5M12 4.875C12 3.42525 13.1753 2.25 14.625 2.25C16.0747 2.25 17.25 3.42525 17.25 4.875C17.25 6.32475 16.0747 7.5 14.625 7.5C13.8905 7.5 12 7.5 12 7.5M12 7.5V21M3.375 11.25H21.375C21.9963 11.25 22.5 10.7463 22.5 10.125V8.625C22.5 8.00368 21.9963 7.5 21.375 7.5H3.375C2.75368 7.5 2.25 8.00368 2.25 8.625V10.125C2.25 10.7463 2.75368 11.25 3.375 11.25Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
