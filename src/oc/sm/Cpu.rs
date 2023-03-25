#[cfg(feature = "OcSmCpu")]
use leptos::*;
#[cfg(feature = "OcSmCpu")]
///This icon requires the feature `OcSmCpu` to be enabled.
#[component]
pub fn Cpu(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M6.5.75V2h3V.75a.75.75 0 0 1 1.5 0V2h1.25c.966 0 1.75.784 1.75 1.75V5h1.25a.75.75 0 0 1 0 1.5H14v3h1.25a.75.75 0 0 1 0 1.5H14v1.25A1.75 1.75 0 0 1 12.25 14H11v1.25a.75.75 0 0 1-1.5 0V14h-3v1.25a.75.75 0 0 1-1.5 0V14H3.75A1.75 1.75 0 0 1 2 12.25V11H.75a.75.75 0 0 1 0-1.5H2v-3H.75a.75.75 0 0 1 0-1.5H2V3.75C2 2.784 2.784 2 3.75 2H5V.75a.75.75 0 0 1 1.5 0Zm5.75 11.75a.25.25 0 0 0 .25-.25v-8.5a.25.25 0 0 0-.25-.25h-8.5a.25.25 0 0 0-.25.25v8.5c0 .138.112.25.25.25ZM5.75 5h4.5a.75.75 0 0 1 .75.75v4.5a.75.75 0 0 1-.75.75h-4.5a.75.75 0 0 1-.75-.75v-4.5A.75.75 0 0 1 5.75 5Zm.75 4.5h3v-3h-3Z"
        /> < title > { title } < / title > < / svg >
    }
}
