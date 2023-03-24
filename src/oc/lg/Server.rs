#[cfg(feature = "OcLgServer")]
use leptos::*;
#[cfg(feature = "OcLgServer")]
///This icon requires the feature `OcLgServer` to be enabled.
#[component]
pub fn Server(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M10.75 6.5a.75.75 0 0 0 0 1.5h6.5a.75.75 0 0 0 0-1.5h-6.5ZM6 7.25a.75.75 0 0 1 .75-.75h.5a.75.75 0 0 1 0 1.5h-.5A.75.75 0 0 1 6 7.25Zm4 9a.75.75 0 0 1 .75-.75h6.5a.75.75 0 0 1 0 1.5h-6.5a.75.75 0 0 1-.75-.75Zm-3.25-.75a.75.75 0 0 0 0 1.5h.5a.75.75 0 0 0 0-1.5h-.5Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.25 2h17.5c.966 0 1.75.784 1.75 1.75v7c0 .372-.116.716-.314 1 .198.284.314.628.314 1v7a1.75 1.75 0 0 1-1.75 1.75H3.25a1.75 1.75 0 0 1-1.75-1.75v-7c0-.358.109-.707.314-1a1.741 1.741 0 0 1-.314-1v-7C1.5 2.784 2.284 2 3.25 2Zm0 10.5a.25.25 0 0 0-.25.25v7c0 .138.112.25.25.25h17.5a.25.25 0 0 0 .25-.25v-7a.25.25 0 0 0-.25-.25Zm0-1.5h17.5a.25.25 0 0 0 .25-.25v-7a.25.25 0 0 0-.25-.25H3.25a.25.25 0 0 0-.25.25v7c0 .138.112.25.25.25Z"
        /> < title > { title } < / title > < / svg >
    }
}
