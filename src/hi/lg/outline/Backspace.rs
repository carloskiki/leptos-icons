#[cfg(feature = "HiLgOutlineBackspace")]
use leptos::*;
#[cfg(feature = "HiLgOutlineBackspace")]
///This icon requires the feature `HiLgOutlineBackspace` to be enabled.
#[component]
pub fn Backspace(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 9.75L14.25 12M14.25 12L16.5 14.25M14.25 12L16.5 9.75M14.25 12L12 14.25M9.42049 19.1705L3.04549 12.7955C2.60615 12.3562 2.60615 11.6438 3.04549 11.2045L9.42049 4.82951C9.63147 4.61853 9.91762 4.5 10.216 4.5L19.5 4.5C20.7426 4.5 21.75 5.50736 21.75 6.75V17.25C21.75 18.4926 20.7426 19.5 19.5 19.5H10.216C9.91762 19.5 9.63147 19.3815 9.42049 19.1705Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
