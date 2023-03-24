#[cfg(feature = "IoArrowUndoCircle")]
use leptos::*;
#[cfg(feature = "IoArrowUndoCircle")]
///This icon requires the feature `IoArrowUndoCircle` to be enabled.
#[component]
pub fn ArrowUndoCircle(
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
        "M256,48C141.13,48,48,141.13,48,256s93.13,208,208,208,208-93.13,208-208S370.87,48,256,48Zm97.67,281.1c-24.07-25.21-51.51-38.68-108.58-38.68v37.32a8.32,8.32,0,0,1-14.05,6L146.58,254a8.2,8.2,0,0,1,0-11.94L231,162.29a8.32,8.32,0,0,1,14.05,6v37.32c88.73,0,117.42,55.64,122.87,117.09C368.65,330.42,359.07,334.75,353.67,329.1Z"
        /> < title > { title } < / title > < / svg >
    }
}
