#[cfg(feature = "SiGoogledomains")]
use leptos::*;
#[cfg(feature = "SiGoogledomains")]
///This icon requires the feature `SiGoogledomains` to be enabled.
#[component]
pub fn Googledomains(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M17.29.817c-.811 0-1.388.469-1.713 1.208L13.491 7.01c-3.121-3.072-8.145-3.015-11.207.102-3.066 3.12-3.048 8.134.072 11.218a7.991 7.991 0 0 0 5.467 2.29l-.597 1.444a.942.942 0 0 0-.054.325c0 .45.379.811.83.793h5.717c.794 0 1.515-.468 1.84-1.208l8.369-20.003A.708.708 0 0 0 24 1.647a.827.827 0 0 0-.83-.83zm-3.787 6.205a7.94 7.94 0 0 1 2.399 5.609c.018 4.365-3.5 7.936-7.864 7.972h-.199Z"
        /> < title > { title } < / title > < / svg >
    }
}
