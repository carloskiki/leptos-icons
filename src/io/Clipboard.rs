#[cfg(feature = "IoClipboard")]
use leptos::*;
#[cfg(feature = "IoClipboard")]
///This icon requires the feature `IoClipboard` to be enabled.
#[component]
pub fn Clipboard(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M368,48H356.59a8,8,0,0,1-7.44-5.08A42.18,42.18,0,0,0,309.87,16H202.13a42.18,42.18,0,0,0-39.28,26.92A8,8,0,0,1,155.41,48H144a64,64,0,0,0-64,64V432a64,64,0,0,0,64,64H368a64,64,0,0,0,64-64V112A64,64,0,0,0,368,48Zm-48.13,64H192.13a16,16,0,0,1,0-32H319.87a16,16,0,0,1,0,32Z"
        /> < title > { title } < / title > < / svg >
    }
}
