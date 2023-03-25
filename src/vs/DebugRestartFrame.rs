#[cfg(feature = "VsDebugRestartFrame")]
use leptos::*;
#[cfg(feature = "VsDebugRestartFrame")]
///This icon requires the feature `VsDebugRestartFrame` to be enabled.
#[component]
pub fn DebugRestartFrame(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M1 10V9h5.207a5.48 5.48 0 0 0-.185 1H1zm6.257-3a5.54 5.54 0 0 1 1.08-1H1v1h6.257zM6.6 13a5.465 5.465 0 0 1-.393-1H1v1h5.6zM15 3v1H1V3h14zm-3.36 10.031a2.531 2.531 0 1 0-2.192-3.797h1.068v.844h-1.97l-.421-.422v-2.25h.844v1.032a3.375 3.375 0 1 1-.423 3.412l.782-.318a2.532 2.532 0 0 0 2.313 1.5z"
        /> < title > { title } < / title > < / svg >
    }
}
