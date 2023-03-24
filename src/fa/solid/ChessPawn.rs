#[cfg(feature = "FaSolidChessPawn")]
use leptos::*;
#[cfg(feature = "FaSolidChessPawn")]
///This icon requires the feature `FaSolidChessPawn` to be enabled.
#[component]
pub fn ChessPawn(
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
        stroke_witdh = "0" style = style viewBox = "0 0 320 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M199.4 224c29.2-18.4 48.6-50.9 48.6-88c0-57.4-46.6-104-104-104S40 78.6 40 136c0 37.1 19.4 69.6 48.5 88H80c-17.7 0-32 14.3-32 32c0 16.5 12.5 30 28.5 31.8L64 400H224L211.5 287.8c16-1.8 28.5-15.3 28.5-31.8c0-17.7-14.3-32-32-32h-8.6zM6.6 473.4c-4.2 4.2-6.6 10-6.6 16C0 501.9 10.1 512 22.6 512H265.4c12.5 0 22.6-10.1 22.6-22.6c0-6-2.4-11.8-6.6-16L240 432H48L6.6 473.4z"
        /> < title > { title } < / title > < / svg >
    }
}
