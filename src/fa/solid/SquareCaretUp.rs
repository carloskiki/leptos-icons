#[cfg(feature = "FaSolidSquareCaretUp")]
use leptos::*;
#[cfg(feature = "FaSolidSquareCaretUp")]
///This icon requires the feature `FaSolidSquareCaretUp` to be enabled.
#[component]
pub fn SquareCaretUp(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M64 32C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64H384c35.3 0 64-28.7 64-64V96c0-35.3-28.7-64-64-64H64zM224 160c6.7 0 13 2.8 17.6 7.7l104 112c6.5 7 8.2 17.2 4.4 25.9s-12.5 14.4-22 14.4H120c-9.5 0-18.2-5.7-22-14.4s-2.1-18.9 4.4-25.9l104-112c4.5-4.9 10.9-7.7 17.6-7.7z"
        /> < title > { title } < / title > < / svg >
    }
}
