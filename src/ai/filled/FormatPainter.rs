#[cfg(feature = "AiFilledFormatPainter")]
use leptos::*;
#[cfg(feature = "AiFilledFormatPainter")]
///This icon requires the feature `AiFilledFormatPainter` to be enabled.
#[component]
pub fn FormatPainter(
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
        stroke_witdh = "0" style = style t = "1569683558243" class = "icon" viewBox =
        "0 0 1024 1024" version = "1.1" p - id = "11409" width = "200" height = "200"
        width = { size.clone() } height = { size } > < defs xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" ><
        style type = "text/css" /></ defs >< path xmlns = "http://www.w3.org/2000/svg"
        xmlns : xlink = "http://www.w3.org/1999/xlink" d =
        "M840 192h-56v-72c0-13.3-10.7-24-24-24H168c-13.3 0-24 10.7-24 24v272c0 13.3 10.7 24 24 24h592c13.3 0 24-10.7 24-24V256h32v200H465c-22.1 0-40 17.9-40 40v136h-44c-4.4 0-8 3.6-8 8v228c0 1.1 0.2 2.2 0.6 3.1-0.4 1.6-0.6 3.2-0.6 4.9 0 46.4 37.6 84 84 84s84-37.6 84-84c0-1.7-0.2-3.3-0.6-4.9 0.4-1 0.6-2 0.6-3.1V640c0-4.4-3.6-8-8-8h-44V520h351c22.1 0 40-17.9 40-40V232c0-22.1-17.9-40-40-40z"
        p - id = "11410" /> < title > { title } < / title > < / svg >
    }
}
