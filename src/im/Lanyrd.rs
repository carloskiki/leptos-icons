#[cfg(feature = "ImLanyrd")]
use leptos::*;
#[cfg(feature = "ImLanyrd")]
///This icon requires the feature `ImLanyrd` to be enabled.
#[component]
pub fn Lanyrd(
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
        "M14.5 0h-13c-0.825 0-1.5 0.675-1.5 1.5v13c0 0.825 0.675 1.5 1.5 1.5h13c0.825 0 1.5-0.675 1.5-1.5v-13c0-0.825-0.675-1.5-1.5-1.5zM12.85 12.012l-5.444 1.781c-1.244 0.406-1.369 0.341-1.931-1.4l-1.375-4.259c-0.328-1.009-1.328-3.728-1.497-4.25-0.313-0.969-0.313-1.022 1.516-1.616 1.431-0.469 1.491-0.453 2.009 1.163 0.419 1.3 0.688 2.35 1.119 3.678l1.172 3.625 3.744-1.225c0.738-0.244 0.984-0.231 1.194 0.678l0.15 0.688c0.175 0.797-0.228 1-0.656 1.137z"
        /> < title > { title } < / title > < / svg >
    }
}
