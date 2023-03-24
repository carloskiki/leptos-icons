#[cfg(feature = "FaSolidTruckFront")]
use leptos::*;
#[cfg(feature = "FaSolidTruckFront")]
///This icon requires the feature `FaSolidTruckFront` to be enabled.
#[component]
pub fn TruckFront(
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
        "M0 80C0 35.8 35.8 0 80 0H432c44.2 0 80 35.8 80 80V368c0 26.2-12.6 49.4-32 64v48c0 17.7-14.3 32-32 32H416c-17.7 0-32-14.3-32-32V448H128v32c0 17.7-14.3 32-32 32H64c-17.7 0-32-14.3-32-32V432C12.6 417.4 0 394.2 0 368V80zm129.9 72.2L112 224H400l-17.9-71.8C378.5 138 365.7 128 351 128H161c-14.7 0-27.5 10-31 24.2zM128 320a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm288 32a32 32 0 1 0 0-64 32 32 0 1 0 0 64z"
        /> < title > { title } < / title > < / svg >
    }
}
