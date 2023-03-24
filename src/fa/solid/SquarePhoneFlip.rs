#[cfg(feature = "FaSolidSquarePhoneFlip")]
use leptos::*;
#[cfg(feature = "FaSolidSquarePhoneFlip")]
///This icon requires the feature `FaSolidSquarePhoneFlip` to be enabled.
#[component]
pub fn SquarePhoneFlip(
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
        "M384 32c35.3 0 64 28.7 64 64V416c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V96C0 60.7 28.7 32 64 32H384zm-90.7 96.7c-9.7-2.6-19.9 2.3-23.7 11.6l-20 48c-3.4 8.2-1 17.6 5.8 23.2L280 231.7c-16.6 35.2-45.1 63.7-80.3 80.3l-20.2-24.7c-5.6-6.8-15-9.2-23.2-5.8l-48 20c-9.3 3.9-14.2 14-11.6 23.7l12 44C111.1 378 119 384 128 384c123.7 0 224-100.3 224-224c0-9-6-16.9-14.7-19.3l-44-12z"
        /> < title > { title } < / title > < / svg >
    }
}
