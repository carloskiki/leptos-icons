#[cfg(feature = "FaSolidHandBackFist")]
use leptos::*;
#[cfg(feature = "FaSolidHandBackFist")]
///This icon requires the feature `FaSolidHandBackFist` to be enabled.
#[component]
pub fn HandBackFist(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M128 0C101.5 0 80 21.5 80 48V96v28.5V176c0 8.8-7.2 16-16 16s-16-7.2-16-16V149.3l-9 7.5C24.4 169 16 187 16 206V244c0 38 16.9 74 46.1 98.3L112 384v96c0 17.7 14.3 32 32 32H304c17.7 0 32-14.3 32-32V374.7c46.9-19 80-65 80-118.7V176 160 144c0-26.5-21.5-48-48-48c-12.4 0-23.6 4.7-32.1 12.3C334 83.5 313.3 64 288 64c-12.4 0-23.6 4.7-32.1 12.3C254 51.5 233.3 32 208 32c-12.4 0-23.6 4.7-32.1 12.3C174 19.5 153.3 0 128 0z"
        /> < title > { title } < / title > < / svg >
    }
}
