#[cfg(feature = "RiHealthLineThermometer")]
use leptos::*;
#[cfg(feature = "RiHealthLineThermometer")]
///This icon requires the feature `RiHealthLineThermometer` to be enabled.
#[component]
pub fn Thermometer(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0H24V24H0z" />< path d =
        "M20.556 3.444c1.562 1.562 1.562 4.094 0 5.657l-8.2 8.2c-.642.642-1.484 1.047-2.387 1.147l-3.378.374-2.298 2.3c-.39.39-1.024.39-1.414 0-.39-.391-.39-1.024 0-1.415l2.298-2.299.375-3.377c.1-.903.505-1.745 1.147-2.387l8.2-8.2c1.563-1.562 4.095-1.562 5.657 0zm-4.242 1.414l-8.2 8.2c-.322.321-.524.742-.574 1.193l-.276 2.485 2.485-.276c.45-.05.872-.252 1.193-.573l.422-.423L9.95 14.05l1.414-1.414 1.414 1.414 1.414-1.414-1.414-1.414 1.414-1.414 1.415 1.414 1.414-1.415-1.414-1.414L17.02 6.98l1.414 1.414.707-.707c.781-.78.781-2.047 0-2.828-.78-.781-2.047-.781-2.828 0z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
