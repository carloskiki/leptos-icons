#[cfg(feature = "ImCloudCheck")]
use leptos::*;
#[cfg(feature = "ImCloudCheck")]
///This icon requires the feature `ImCloudCheck` to be enabled.
#[component]
pub fn CloudCheck(
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
        "M13.942 8.039c0.038-0.174 0.058-0.354 0.058-0.539 0-1.381-1.119-2.5-2.5-2.5-0.222 0-0.438 0.029-0.643 0.084-0.387-1.209-1.52-2.084-2.857-2.084-1.365 0-2.516 0.911-2.88 2.159-0.355-0.103-0.731-0.159-1.12-0.159-2.209 0-4 1.791-4 4s1.791 4 4 4h9.5c1.381 0 2.5-1.119 2.5-2.5 0-1.23-0.888-2.252-2.058-2.461zM6.5 12l-2.5-2.5 1-1 1.5 1.5 3.5-3.5 1 1-4.5 4.5z"
        /> < title > { title } < / title > < / svg >
    }
}
