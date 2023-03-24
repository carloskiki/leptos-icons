#[cfg(feature = "OcSmFiscalHost")]
use leptos::*;
#[cfg(feature = "OcSmFiscalHost")]
///This icon requires the feature `OcSmFiscalHost` to be enabled.
#[component]
pub fn FiscalHost(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 8a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M4 9.25h-.75a.75.75 0 0 1 0-1.5H4v-1.5h-.75a.75.75 0 0 1 0-1.5H4V3.5a1 1 0 0 1 1-1h7.5a1 1 0 0 1 1 1v7a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1ZM5.5 4v.793a.75.75 0 0 1 0 1.414v1.586a.75.75 0 0 1 0 1.414V10H12V4Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.75 14.25V14h-9.5v.25a.75.75 0 0 1-1.5 0V14A1.75 1.75 0 0 1 0 12.25V1.75C0 .784.784 0 1.75 0h12.5C15.217 0 16 .784 16 1.75v10.5A1.75 1.75 0 0 1 14.25 14v.25a.75.75 0 0 1-1.5 0ZM1.75 1.5a.25.25 0 0 0-.25.25v10.5c0 .138.112.25.25.25h12.5a.25.25 0 0 0 .25-.25V1.75a.25.25 0 0 0-.25-.25Z"
        /> < title > { title } < / title > < / svg >
    }
}
