#[cfg(feature = "FaSolidPlantWilt")]
use leptos::*;
#[cfg(feature = "FaSolidPlantWilt")]
///This icon requires the feature `FaSolidPlantWilt` to be enabled.
#[component]
pub fn PlantWilt(
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
        "M288 120c0-30.9 25.1-56 56-56s56 25.1 56 56v13c-29.3 10-48 34.5-48 70.1c0 27.9 25.3 74.8 66 111.6c3.8 3.5 8.9 5.3 14 5.3s10.2-1.8 14-5.3c40.7-36.8 66-83.7 66-111.6c0-35.6-18.7-60.2-48-70.1V120C464 53.7 410.3 0 344 0S224 53.7 224 120v21.8C207.3 133 188.2 128 168 128c-66.3 0-120 53.7-120 120v13c-29.3 10-48 34.5-48 70.1C0 359 25.3 405.9 66 442.7c3.8 3.5 8.9 5.3 14 5.3s10.2-1.8 14-5.3c40.7-36.8 66-83.7 66-111.6c0-35.6-18.7-60.2-48-70.1V248c0-30.9 25.1-56 56-56s56 25.1 56 56v32V480c0 17.7 14.3 32 32 32s32-14.3 32-32V280 248 120z"
        /> < title > { title } < / title > < / svg >
    }
}
