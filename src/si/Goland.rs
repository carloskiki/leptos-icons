#[cfg(feature = "SiGoland")]
use leptos::*;
#[cfg(feature = "SiGoland")]
///This icon requires the feature `SiGoland` to be enabled.
#[component]
pub fn Goland(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M0 0v24h24V0Zm6.764 3a5.448 5.448 0 0 1 3.892 1.356L9.284 6.012A3.652 3.652 0 0 0 6.696 5c-1.6 0-2.844 1.4-2.844 3.08v.028c0 1.812 1.244 3.14 3 3.14a3.468 3.468 0 0 0 2.048-.596V9.228H6.708v-1.88H11v4.296a6.428 6.428 0 0 1-4.228 1.572c-3.076 0-5.196-2.164-5.196-5.092v-.028A5.08 5.08 0 0 1 6.764 3Zm10.432 0c3.052 0 5.244 2.276 5.244 5.088v.028a5.116 5.116 0 0 1-5.272 5.12c-3.056-.02-5.248-2.296-5.248-5.112v-.028A5.116 5.116 0 0 1 17.196 3Zm-.028 2A2.96 2.96 0 0 0 14.2 8.068v.028a3.008 3.008 0 0 0 3 3.112 2.96 2.96 0 0 0 2.964-3.084v-.028A3.004 3.004 0 0 0 17.168 5ZM2.252 19.5h9V21h-9z"
        /> < title > { title } < / title > < / svg >
    }
}
