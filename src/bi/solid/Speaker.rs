#[cfg(feature = "BiSolidSpeaker")]
use leptos::*;
#[cfg(feature = "BiSolidSpeaker")]
///This icon requires the feature `BiSolidSpeaker` to be enabled.
#[component]
pub fn Speaker(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > <
        circle xmlns = "http://www.w3.org/2000/svg" cx = "12" cy = "15" r = "3" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M18 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2zm-6 2a2 2 0 1 1-2 2 2 2 0 0 1 2-2zm0 16a5 5 0 1 1 5-5 5 5 0 0 1-5 5z"
        /> < title > { title } < / title > < / svg >
    }
}
