#[cfg(feature = "SiFozzy")]
use leptos::*;
#[cfg(feature = "SiFozzy")]
///This icon requires the feature `SiFozzy` to be enabled.
#[component]
pub fn Fozzy(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M14.494 20.48l-.998-2.095 5.787-11.273c.897 1.396 1.496 3.092 1.496 4.888 0 3.99-2.594 7.382-6.285 8.48zM12.998.029C5.615-.471-.47 5.615.028 12.998c.5 5.786 5.188 10.475 10.974 10.973 7.383.5 13.468-5.586 12.97-12.969C23.471 5.216 18.783.527 12.997.03zM7.112 4.717c1.297-.897 2.793-1.396 4.39-1.496L8.807 8.409 7.112 4.717zm3.491 7.383l4.19-8.38c.798.3 1.497.598 2.195 1.097L11.9 14.793 10.603 12.1zM3.221 12c0-1.796.599-3.492 1.496-4.888l6.485 13.667C6.712 20.38 3.22 16.589 3.22 12z"
        /> < title > { title } < / title > < / svg >
    }
}
