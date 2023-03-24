#[cfg(feature = "RiHealthFillThermometer")]
use leptos::*;
#[cfg(feature = "RiHealthFillThermometer")]
///This icon requires the feature `RiHealthFillThermometer` to be enabled.
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M20.556 3.444c1.562 1.562 1.562 4.094 0 5.657l-8.2 8.2c-.642.642-1.484 1.047-2.387 1.147l-3.378.374-2.298 2.3c-.39.39-1.024.39-1.414 0-.39-.391-.39-1.024 0-1.415l2.298-2.299.375-3.377c.1-.903.505-1.745 1.147-2.387l8.2-8.2c1.563-1.562 4.095-1.562 5.657 0zm-9.192 9.192L9.95 14.05l2.121 2.122 1.414-1.415-2.121-2.121zm2.828-2.828l-1.414 1.414 2.121 2.121 1.415-1.414-2.122-2.121zm2.829-2.829l-1.414 1.414 2.12 2.122L19.143 9.1l-2.121-2.122z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
