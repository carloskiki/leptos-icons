#[cfg(feature = "TiArrowRightOutline")]
use leptos::*;
#[cfg(feature = "TiArrowRightOutline")]
///This icon requires the feature `TiArrowRightOutline` to be enabled.
#[component]
pub fn ArrowRightOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 21c-.801 0-1.555-.312-2.121-.879s-.88-1.321-.879-2.123c0-.746.271-.998.764-1.998h-4.836c-1.654 0-3-1.347-3-3 0-1.654 1.346-3 3-3h4.836c-.494-1-.764-1.255-.764-2.001.001-.802.312-1.554.88-2.121 1.132-1.132 3.108-1.133 4.241.001l7.121 7.121-7.121 7.121c-.566.567-1.32.879-2.121.879zm-7.072-9c-.552 0-1 .449-1 1s.448 1 1 1h9.658l-3.293 3.293c-.189.189-.293.439-.293.706 0 .269.104.519.293.708.379.378 1.035.378 1.414 0l5.707-5.707-5.707-5.707c-.379-.378-1.035-.378-1.414 0-.189.189-.293.439-.293.706 0 .268.104.519.293.708l3.293 3.293h-9.658z"
        /> < title > { title } < / title > < / svg >
    }
}
