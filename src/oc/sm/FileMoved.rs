#[cfg(feature = "OcSmFileMoved")]
use leptos::*;
#[cfg(feature = "OcSmFileMoved")]
///This icon requires the feature `OcSmFileMoved` to be enabled.
#[component]
pub fn FileMoved(
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
        "M2 1.75C2 .784 2.784 0 3.75 0h6.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v9.586A1.75 1.75 0 0 1 13.25 16h-3.5a.75.75 0 0 1 0-1.5h3.5a.25.25 0 0 0 .25-.25V4.664a.25.25 0 0 0-.073-.177l-2.914-2.914a.25.25 0 0 0-.177-.073H3.75a.25.25 0 0 0-.25.25v6.5a.75.75 0 0 1-1.5 0v-6.5Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "m5.427 15.573 3.146-3.146a.25.25 0 0 0 0-.354L5.427 8.927A.25.25 0 0 0 5 9.104V11.5H.75a.75.75 0 0 0 0 1.5H5v2.396c0 .223.27.335.427.177Z"
        /> < title > { title } < / title > < / svg >
    }
}
