#[cfg(feature = "SiSmart")]
use leptos::*;
#[cfg(feature = "SiSmart")]
///This icon requires the feature `SiSmart` to be enabled.
#[component]
pub fn Smart(
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
        "M17.048 2.611c.318 3.819.248 16.941.012 18.605-.016.125-.018.341.113.239A23.879 23.879 0 0 0 24 12.003c-1.467-3.684-3.722-6.74-6.604-9.258-.167-.147-.267-.229-.314-.229-.03 0-.04.03-.034.095M0 12.002c0 6.215 5.064 11.252 11.245 11.252 1.612 0 3.138-.35 4.332-.86.34-.146.32-.377.325-.478l.001-4.945c0-.244-.192-.139-.29-.087-2.292 1.228-4.374 1.15-4.374 1.15-3.416 0-6.032-2.797-6.032-6.03 0-3.306 2.671-6.055 6.049-6.055 1.788 0 3.19.56 4.385 1.164.106.054.256.042.263-.117l-.002-4.729s.023-.53-.42-.705C13.951.96 12.584.746 11.26.746A11.245 11.245 0 0 0 0 12.002Z"
        /> < title > { title } < / title > < / svg >
    }
}
