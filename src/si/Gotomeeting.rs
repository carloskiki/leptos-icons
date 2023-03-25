#[cfg(feature = "SiGotomeeting")]
use leptos::*;
#[cfg(feature = "SiGotomeeting")]
///This icon requires the feature `SiGotomeeting` to be enabled.
#[component]
pub fn Gotomeeting(
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
        "M21.3 13.72a3.158 3.158 0 0 0-3.462.124.632.632 0 0 1-.682.035l-3.137-1.81a.08.08 0 0 1 0-.137l3.12-1.8a.632.632 0 0 1 .685.036 3.158 3.158 0 0 0 3.47.139A3.194 3.194 0 0 0 22.442 6.1a3.158 3.158 0 0 0-5.924 1.773.633.633 0 0 1-.311.606l-3.136 1.811a.08.08 0 0 1-.118-.068V6.6a.632.632 0 0 1 .372-.573 3.158 3.158 0 1 0-2.64 0 .632.632 0 0 1 .373.573v3.622a.08.08 0 0 1-.119.068L7.804 8.48a.632.632 0 0 1-.311-.605 3.157 3.157 0 1 0-1.307 2.294.633.633 0 0 1 .687-.038l3.12 1.8a.08.08 0 0 1 0 .137L6.854 13.88a.632.632 0 0 1-.683-.035 3.158 3.158 0 0 0-3.461-.124 3.194 3.194 0 0 0-1.143 4.202 3.159 3.159 0 0 0 5.924-1.792.633.633 0 0 1 .31-.61l3.137-1.81a.08.08 0 0 1 .119.068V17.4a.632.632 0 0 1-.372.573 3.158 3.158 0 1 0 2.64 0 .633.633 0 0 1-.373-.573v-3.621a.08.08 0 0 1 .118-.069l3.137 1.812a.631.631 0 0 1 .31.609 3.159 3.159 0 0 0 5.924 1.792A3.194 3.194 0 0 0 21.3 13.72Z"
        /> < title > { title } < / title > < / svg >
    }
}
