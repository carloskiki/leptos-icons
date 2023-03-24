#[cfg(feature = "ImDrawer2")]
use leptos::*;
#[cfg(feature = "ImDrawer2")]
///This icon requires the feature `ImDrawer2` to be enabled.
#[component]
pub fn Drawer2(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M15.89 10.188l-4-5c-0.095-0.119-0.239-0.188-0.39-0.188h-7c-0.152 0-0.296 0.069-0.39 0.188l-4 5c-0.071 0.089-0.11 0.199-0.11 0.312v4.5c0 0.552 0.448 1 1 1h14c0.552 0 1-0.448 1-1v-4.5c0-0.114-0.039-0.224-0.11-0.312zM15 11h-3.5l-2 2h-3l-2-2h-3.5v-0.325l3.74-4.675h6.519l3.74 4.675v0.325z"
        /> < title > { title } < / title > < / svg >
    }
}
