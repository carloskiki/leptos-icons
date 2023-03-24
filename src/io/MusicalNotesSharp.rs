#[cfg(feature = "IoMusicalNotesSharp")]
use leptos::*;
#[cfg(feature = "IoMusicalNotesSharp")]
///This icon requires the feature `IoMusicalNotesSharp` to be enabled.
#[component]
pub fn MusicalNotesSharp(
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
        "M429.46,32.07c-23.6,6.53-205.55,58.81-250.54,71.43a4,4,0,0,0-2.92,3.83v247a2,2,0,0,1-1.33,1.89l-27.85,9.55c-19,7.44-66.82,16.68-66.82,59.19,0,35.54,24.63,51.54,45.86,54.28a52.06,52.06,0,0,0,7.81.8c7.37,0,36.38-7.08,53.3-18.08C208,448.25,208,448,208,412V202c0-.9.62-.84,1.48-1.07l188-51.92a2,2,0,0,1,2.53,2V306.55a2,2,0,0,1-1.36,1.89c-8.9,3-19.23,6.5-26.48,9.12C341.39,328.68,304,335.65,304,376c0,38.51,28.26,54.58,46.3,55.83a87.37,87.37,0,0,0,21.33-1c9-1.38,24.09-5.9,38.14-14.86C432,401.79,432,401.51,432,360V34A2,2,0,0,0,429.46,32.07Z"
        /> < title > { title } < / title > < / svg >
    }
}
