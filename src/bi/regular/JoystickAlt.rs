#[cfg(feature = "BiRegularJoystickAlt")]
use leptos::*;
#[cfg(feature = "BiRegularJoystickAlt")]
///This icon requires the feature `BiRegularJoystickAlt` to be enabled.
#[component]
pub fn JoystickAlt(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < circle xmlns =
        "http://www.w3.org/2000/svg" cx = "15" cy = "13" r = "1" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "17" cy = "11" r = "1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 9H8v2H6v2h2v2h2v-2h2v-2h-2z" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M15 5H9a7 7 0 0 0-7 7 7 7 0 0 0 7 7h6a7 7 0 0 0 7-7 7 7 0 0 0-7-7zm0 12H9A5 5 0 1 1 9 7h6a5 5 0 1 1 0 10z"
        /> < title > { title } < / title > < / svg >
    }
}
