#[cfg(feature = "AiFilledMedicineBox")]
use leptos::*;
#[cfg(feature = "AiFilledMedicineBox")]
///This icon requires the feature `AiFilledMedicineBox` to be enabled.
#[component]
pub fn MedicineBox(
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
        stroke_witdh = "0" style = style class = "icon" viewBox = "0 0 1024 1024" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M839.2 278.1a32 32 0 0 0-30.4-22.1H736V144c0-17.7-14.3-32-32-32H320c-17.7 0-32 14.3-32 32v112h-72.8a31.9 31.9 0 0 0-30.4 22.1L112 502v378c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V502l-72.8-223.9zM660 628c0 4.4-3.6 8-8 8H544v108c0 4.4-3.6 8-8 8h-48c-4.4 0-8-3.6-8-8V636H372c-4.4 0-8-3.6-8-8v-48c0-4.4 3.6-8 8-8h108V464c0-4.4 3.6-8 8-8h48c4.4 0 8 3.6 8 8v108h108c4.4 0 8 3.6 8 8v48zm4-372H360v-72h304v72z"
        /> < title > { title } < / title > < / svg >
    }
}
