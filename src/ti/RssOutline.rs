#[cfg(feature = "TiRssOutline")]
use leptos::*;
#[cfg(feature = "TiRssOutline")]
///This icon requires the feature `TiRssOutline` to be enabled.
#[component]
pub fn RssOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 4.999c-1.657 0-3.011 1.344-3.011 3l.005 9c0 2.209 1.793 4 4.002 4l9.003.001c1.655 0 3-1.346 3-3.001.001-7.179-5.819-13-12.999-13zm1.001 14c-1.105.002-2.001-.894-2.001-1.999-.002-1.105.894-2.001 2.001-2.001 1.105-.002 2.001.894 1.999 2.001.002 1.105-.894 2.001-1.999 1.999zm4.499 0c-.829 0-1.5-.671-1.5-1.5 0-1.931-1.57-3.5-3.5-3.5-.829 0-1.5-.671-1.5-1.5s.671-1.5 1.5-1.5c3.584 0 6.5 2.916 6.5 6.5 0 .829-.671 1.5-1.5 1.5zm4 0c-.829 0-1.5-.671-1.5-1.5 0-4.136-3.364-7.5-7.5-7.5-.829 0-1.5-.671-1.5-1.5s.671-1.5 1.5-1.5c5.79 0 10.5 4.71 10.5 10.5 0 .829-.671 1.5-1.5 1.5z"
        /> < title > { title } < / title > < / svg >
    }
}
