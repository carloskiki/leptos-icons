#[cfg(feature = "RiMediaFillCameraOff")]
use leptos::*;
#[cfg(feature = "RiMediaFillCameraOff")]
///This icon requires the feature `RiMediaFillCameraOff` to be enabled.
#[component]
pub fn CameraOff(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M19.586 21H3a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1h.586L1.393 2.808l1.415-1.415 19.799 19.8-1.415 1.414L19.586 21zM7.556 8.97a6 6 0 0 0 8.475 8.475l-1.417-1.417a4 4 0 0 1-5.642-5.642L7.555 8.97zM22 17.785l-4.045-4.045a6 6 0 0 0-6.695-6.695L8.106 3.892 9 3h6l2 2h4a1 1 0 0 1 1 1v11.786zm-8.492-8.492a4.013 4.013 0 0 1 2.198 2.198l-2.198-2.198z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
