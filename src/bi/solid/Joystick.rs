#[cfg(feature = "BiSolidJoystick")]
use leptos::*;
#[cfg(feature = "BiSolidJoystick")]
///This icon requires the feature `BiSolidJoystick` to be enabled.
#[component]
pub fn Joystick(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M21.986 9.74a3.193 3.193 0 0 0-.008-.088A5.003 5.003 0 0 0 17 5H7a4.97 4.97 0 0 0-4.987 4.737c-.01.079-.013.161-.013.253v6.51c0 .925.373 1.828 1.022 2.476A3.524 3.524 0 0 0 5.5 20c1.8 0 2.504-1 3.5-3 .146-.292.992-2 3-2 1.996 0 2.853 1.707 3 2 1.004 2 1.7 3 3.5 3 .925 0 1.828-.373 2.476-1.022A3.524 3.524 0 0 0 22 16.5V10c0-.095-.004-.18-.014-.26zM7 12.031a2 2 0 1 1-.001-3.999A2 2 0 0 1 7 12.031zm10-5a1 1 0 1 1 0 2 1 1 0 1 1 0-2zm-2 4a1 1 0 1 1 0-2 1 1 0 1 1 0 2zm2 2a1 1 0 1 1 0-2 1 1 0 1 1 0 2zm2-2a1 1 0 1 1 0-2 1 1 0 1 1 0 2z"
        /> < title > { title } < / title > < / svg >
    }
}
