#[cfg(feature = "ImRss2")]
use leptos::*;
#[cfg(feature = "ImRss2")]
///This icon requires the feature `ImRss2` to be enabled.
#[component]
pub fn Rss2(
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
        "M14.5 0h-13c-0.825 0-1.5 0.675-1.5 1.5v13c0 0.825 0.675 1.5 1.5 1.5h13c0.825 0 1.5-0.675 1.5-1.5v-13c0-0.825-0.675-1.5-1.5-1.5zM4.359 12.988c-0.75 0-1.359-0.603-1.359-1.353 0-0.744 0.609-1.356 1.359-1.356 0.753 0 1.359 0.613 1.359 1.356 0 0.75-0.609 1.353-1.359 1.353zM7.772 13c0-1.278-0.497-2.481-1.397-3.381-0.903-0.903-2.1-1.4-3.375-1.4v-1.956c3.713 0 6.738 3.022 6.738 6.737h-1.966zM11.244 13c0-4.547-3.697-8.25-8.241-8.25v-1.956c5.625 0 10.203 4.581 10.203 10.206h-1.963z"
        /> < title > { title } < / title > < / svg >
    }
}
