#[cfg(feature = "HiLgOutlineListBullet")]
use leptos::*;
#[cfg(feature = "HiLgOutlineListBullet")]
///This icon requires the feature `HiLgOutlineListBullet` to be enabled.
#[component]
pub fn ListBullet(
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
        "M8.25 6.75H20.25M8.25 12H20.25M8.25 17.25H20.25M3.75 6.75H3.7575V6.7575H3.75V6.75ZM4.125 6.75C4.125 6.95711 3.95711 7.125 3.75 7.125C3.54289 7.125 3.375 6.95711 3.375 6.75C3.375 6.54289 3.54289 6.375 3.75 6.375C3.95711 6.375 4.125 6.54289 4.125 6.75ZM3.75 12H3.7575V12.0075H3.75V12ZM4.125 12C4.125 12.2071 3.95711 12.375 3.75 12.375C3.54289 12.375 3.375 12.2071 3.375 12C3.375 11.7929 3.54289 11.625 3.75 11.625C3.95711 11.625 4.125 11.7929 4.125 12ZM3.75 17.25H3.7575V17.2575H3.75V17.25ZM4.125 17.25C4.125 17.4571 3.95711 17.625 3.75 17.625C3.54289 17.625 3.375 17.4571 3.375 17.25C3.375 17.0429 3.54289 16.875 3.75 16.875C3.95711 16.875 4.125 17.0429 4.125 17.25Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
