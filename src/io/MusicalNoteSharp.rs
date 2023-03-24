#[cfg(feature = "IoMusicalNoteSharp")]
use leptos::*;
#[cfg(feature = "IoMusicalNoteSharp")]
///This icon requires the feature `IoMusicalNoteSharp` to be enabled.
#[component]
pub fn MusicalNoteSharp(
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
        "M381.55,32.05c-18.13,4.28-126.57,31.07-156,38.19A2,2,0,0,0,224,72.18V353.3a2,2,0,0,1-1.32,1.88L182,369.88c-29.82,10.66-54,18.94-54,59.06,0,32.47,23.53,47.18,37.95,50a81.77,81.77,0,0,0,15,1.08c8.89,0,31-3.59,47.52-14.24C256,448,256,448,256,415.93V169.16a2,2,0,0,1,1.49-1.94l125-33a2,2,0,0,0,1.49-1.94V34A2,2,0,0,0,381.55,32.05Z"
        /> < title > { title } < / title > < / svg >
    }
}
