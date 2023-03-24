#[cfg(feature = "FaSolidOutdent")]
use leptos::*;
#[cfg(feature = "FaSolidOutdent")]
///This icon requires the feature `FaSolidOutdent` to be enabled.
#[component]
pub fn Outdent(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 64C6 46.3 20.3 32 38 32H422c17.7 0 32 14.3 32 32s-14.3 32-32 32H38C20.3 96 6 81.7 6 64zM198 192c0-17.7 14.3-32 32-32H422c17.7 0 32 14.3 32 32s-14.3 32-32 32H230c-17.7 0-32-14.3-32-32zm32 96H422c17.7 0 32 14.3 32 32s-14.3 32-32 32H230c-17.7 0-32-14.3-32-32s14.3-32 32-32zM6 448c0-17.7 14.3-32 32-32H422c17.7 0 32 14.3 32 32s-14.3 32-32 32H38c-17.7 0-32-14.3-32-32zm.2-179.4c-8.2-6.4-8.2-18.9 0-25.3l101.9-79.3c10.5-8.2 25.8-.7 25.8 12.6V335.3c0 13.3-15.3 20.8-25.8 12.6L6.2 268.6z"
        /> < title > { title } < / title > < / svg >
    }
}
