#[cfg(feature = "HiMdSolidClipboard")]
use leptos::*;
#[cfg(feature = "HiMdSolidClipboard")]
///This icon requires the feature `HiMdSolidClipboard` to be enabled.
#[component]
pub fn Clipboard(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M13.8871 3.18189C14.283 3.21913 14.6773 3.262 15.07 3.31043C16.1942 3.44911 17 4.41371 17 5.51659V16.75C17 17.9926 15.9926 19 14.75 19H5.25C4.00736 19 3 17.9926 3 16.75V5.51659C3 4.41371 3.80579 3.44911 4.93005 3.31043C5.32266 3.26201 5.71697 3.21913 6.11291 3.18189C6.46903 1.92267 7.62676 1 9 1H11C12.3732 1 13.531 1.92267 13.8871 3.18189ZM7.5 4C7.5 3.17157 8.17157 2.5 9 2.5H11C11.8284 2.5 12.5 3.17157 12.5 4V4.5H7.5V4Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
