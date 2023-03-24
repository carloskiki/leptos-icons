#[cfg(feature = "ImSpoonKnife")]
use leptos::*;
#[cfg(feature = "ImSpoonKnife")]
///This icon requires the feature `ImSpoonKnife` to be enabled.
#[component]
pub fn SpoonKnife(
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
        "M3.5 0c-1.657 0-3 1.567-3 3.5 0 1.655 0.985 3.042 2.308 3.406l-0.497 8.096c-0.034 0.549 0.389 0.998 0.939 0.998h0.5c0.55 0 0.972-0.449 0.939-0.998l-0.497-8.096c1.323-0.365 2.308-1.751 2.308-3.406 0-1.933-1.343-3.5-3-3.5zM13.583 0l-0.833 5h-0.625l-0.417-5h-0.417l-0.417 5h-0.625l-0.833-5h-0.417v6.5c0 0.276 0.224 0.5 0.5 0.5h1.302l-0.491 8.002c-0.034 0.549 0.389 0.998 0.939 0.998h0.5c0.55 0 0.972-0.449 0.939-0.998l-0.491-8.002h1.302c0.276 0 0.5-0.224 0.5-0.5v-6.5h-0.417z"
        /> < title > { title } < / title > < / svg >
    }
}
