#[cfg(feature = "ImDropbox")]
use leptos::*;
#[cfg(feature = "ImDropbox")]
///This icon requires the feature `ImDropbox` to be enabled.
#[component]
pub fn Dropbox(
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
        "M11.5 0.5l-3.5 3 4.5 3 3.5-3z" />< path xmlns = "http://www.w3.org/2000/svg"
        xmlns : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M8 3.5l-3.5-3-4.5 3 3.5 3z" />< path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M12.5 6.5l3.5 3-4.5 2.5-3.5-3z" />< path xmlns = "http://www.w3.org/2000/svg"
        xmlns : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M8 9l-4.5-2.5-3.5 3 4.5 2.5z" />< path xmlns = "http://www.w3.org/2000/svg"
        xmlns : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M11.377 13.212l-3.377-2.895-3.377 2.895-2.123-1.179v1.467l5.5 2.5 5.5-2.5v-1.467z"
        /> < title > { title } < / title > < / svg >
    }
}
