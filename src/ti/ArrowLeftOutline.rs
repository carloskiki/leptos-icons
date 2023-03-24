#[cfg(feature = "TiArrowLeftOutline")]
use leptos::*;
#[cfg(feature = "TiArrowLeftOutline")]
///This icon requires the feature `TiArrowLeftOutline` to be enabled.
#[component]
pub fn ArrowLeftOutline(
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
        "M10.928 21c-.801 0-1.555-.312-2.121-.879l-7.121-7.121 7.121-7.121c1.133-1.134 3.109-1.134 4.242 0 .566.564.879 1.317.879 2.119 0 .746-.27 1.451-.764 2.002h4.836c1.654 0 3 1.346 3 3s-1.346 3-3 3h-4.836c.493.549.764 1.252.764 1.998.002.802-.312 1.557-.879 2.124-.567.566-1.32.878-2.121.878zm-6.414-8l5.707 5.707c.379.378 1.035.378 1.414 0 .189-.189.293-.441.293-.708s-.104-.517-.291-.705l-3.295-3.294h9.658c.552 0 1-.449 1-1s-.448-1-1-1h-9.658l3.293-3.293c.189-.189.293-.441.293-.708s-.104-.517-.292-.705c-.381-.38-1.036-.379-1.415-.001l-5.707 5.707z"
        /> < title > { title } < / title > < / svg >
    }
}
