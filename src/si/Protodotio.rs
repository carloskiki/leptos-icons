#[cfg(feature = "SiProtodotio")]
use leptos::*;
#[cfg(feature = "SiProtodotio")]
///This icon requires the feature `SiProtodotio` to be enabled.
#[component]
pub fn Protodotio(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 11.997c-.728 0-1.316-.59-1.316-1.317S11.272 9.363 12 9.363s1.316.589 1.316 1.316-.588 1.318-1.316 1.318zm2.916-.021c0-2.828-1.109-5.397-2.916-7.298-1.807 1.9-2.916 4.47-2.916 7.298 0 1.297.234 2.535.66 3.683-.618.9-1.074 2.16-1.275 3.616.639-.767 1.422-1.306 2.292-1.591.363.555.78 1.096 1.239 1.574.461-.494.876-1.02 1.239-1.59.87.271 1.653.826 2.29 1.576-.199-1.456-.655-2.716-1.275-3.615.427-1.155.66-2.385.66-3.69l.002.037zM12 22C6.477 22 2 17.523 2 12S6.477 2 12 2s10 4.477 10 10-4.477 10-10 10zm0-22C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0z"
        /> < title > { title } < / title > < / svg >
    }
}
