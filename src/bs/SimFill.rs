#[cfg(feature = "BsSimFill")]
use leptos::*;
#[cfg(feature = "BsSimFill")]
///This icon requires the feature `BsSimFill` to be enabled.
#[component]
pub fn SimFill(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-sim-fill" viewBox = "0 0 16 16" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M5 4.5a.5.5 0 0 1 .5-.5h2v2H5V4.5zM8.5 6V4h2a.5.5 0 0 1 .5.5V6H8.5zM5 7h6v2H5V7zm3.5 3H11v1.5a.5.5 0 0 1-.5.5h-2v-2zm-1 0v2h-2a.5.5 0 0 1-.5-.5V10h2.5z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.5 0A1.5 1.5 0 0 0 2 1.5v13A1.5 1.5 0 0 0 3.5 16h9a1.5 1.5 0 0 0 1.5-1.5V3.414a1.5 1.5 0 0 0-.44-1.06L11.647.439A1.5 1.5 0 0 0 10.586 0H3.5zm2 3h5A1.5 1.5 0 0 1 12 4.5v7a1.5 1.5 0 0 1-1.5 1.5h-5A1.5 1.5 0 0 1 4 11.5v-7A1.5 1.5 0 0 1 5.5 3z"
        /> < title > { title } < / title > < / svg >
    }
}
