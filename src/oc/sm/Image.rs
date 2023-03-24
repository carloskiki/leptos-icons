#[cfg(feature = "OcSmImage")]
use leptos::*;
#[cfg(feature = "OcSmImage")]
///This icon requires the feature `OcSmImage` to be enabled.
#[component]
pub fn Image(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M16 13.25A1.75 1.75 0 0 1 14.25 15H1.75A1.75 1.75 0 0 1 0 13.25V2.75C0 1.784.784 1 1.75 1h12.5c.966 0 1.75.784 1.75 1.75ZM1.75 2.5a.25.25 0 0 0-.25.25v10.5c0 .138.112.25.25.25h.94l.03-.03 6.077-6.078a1.75 1.75 0 0 1 2.412-.06L14.5 10.31V2.75a.25.25 0 0 0-.25-.25Zm12.5 11a.25.25 0 0 0 .25-.25v-.917l-4.298-3.889a.25.25 0 0 0-.344.009L4.81 13.5ZM7 6a2 2 0 1 1-3.999.001A2 2 0 0 1 7 6ZM5.5 6a.5.5 0 1 0-1 0 .5.5 0 0 0 1 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
