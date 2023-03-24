#[cfg(feature = "BsWebcamFill")]
use leptos::*;
#[cfg(feature = "BsWebcamFill")]
///This icon requires the feature `BsWebcamFill` to be enabled.
#[component]
pub fn WebcamFill(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-webcam-fill" viewBox = "0 0 16 16" width = size.clone() height =
        size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M6.644 11.094a.5.5 0 0 1 .356-.15h2a.5.5 0 0 1 .356.15c.175.177.39.347.603.496a7.166 7.166 0 0 0 .752.456l.01.006h.003A.5.5 0 0 1 10.5 13h-5a.5.5 0 0 1-.224-.947l.002-.001.01-.006a3.517 3.517 0 0 0 .214-.116 7.5 7.5 0 0 0 .539-.34c.214-.15.428-.319.603-.496ZM7 6.5a1 1 0 1 1 2 0 1 1 0 0 1-2 0Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 3a2 2 0 0 0-2 2v3a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H2Zm6 1.5a2 2 0 1 1 0 4 2 2 0 0 1 0-4ZM12.5 7a.5.5 0 1 1 0-1 .5.5 0 0 1 0 1Z"
        /> < title > { title } < / title > < / svg >
    }
}
