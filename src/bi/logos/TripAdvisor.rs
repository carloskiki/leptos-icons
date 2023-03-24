#[cfg(feature = "BiLogosTripAdvisor")]
use leptos::*;
#[cfg(feature = "BiLogosTripAdvisor")]
///This icon requires the feature `BiLogosTripAdvisor` to be enabled.
#[component]
pub fn TripAdvisor(
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
        "M8.31 10.28a2.5 2.5 0 1 0 2.5 2.49 2.5 2.5 0 0 0-2.5-2.49zm0 3.8a1.31 1.31 0 1 1 0-2.61 1.31 1.31 0 1 1 0 2.61zm7.38-3.8a2.5 2.5 0 1 0 2.5 2.49 2.5 2.5 0 0 0-2.5-2.49zM17 12.77a1.31 1.31 0 1 1-1.31-1.3 1.31 1.31 0 0 1 1.31 1.3z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2zm7.38 10.77a3.69 3.69 0 0 1-6.2 2.71L12 16.77l-1.18-1.29a3.69 3.69 0 1 1-5-5.44l-1.2-1.3H7.3a8.33 8.33 0 0 1 9.41 0h2.67l-1.2 1.31a3.71 3.71 0 0 1 1.2 2.72z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.77 9.05a7.19 7.19 0 0 0-5.54 0A4.06 4.06 0 0 1 12 12.7a4.08 4.08 0 0 1 2.77-3.65z"
        /> < title > { title } < / title > < / svg >
    }
}
