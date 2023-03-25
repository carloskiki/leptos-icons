#[cfg(feature = "ImMagicWand")]
use leptos::*;
#[cfg(feature = "ImMagicWand")]
///This icon requires the feature `ImMagicWand` to be enabled.
#[component]
pub fn MagicWand(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M4 3l-2-2h-1v1l2 2zM5 0h1v2h-1zM9 5h2v1h-2zM10 2v-1h-1l-2 2 1 1zM0 5h2v1h-2zM5 9h1v2h-1zM1 9v1h1l2-2-1-1zM15.781 13.781l-9.939-9.939c-0.292-0.292-0.769-0.292-1.061 0l-0.939 0.939c-0.292 0.292-0.292 0.769 0 1.061l9.939 9.939c0.292 0.292 0.769 0.292 1.061 0l0.939-0.939c0.292-0.292 0.292-0.769 0-1.061zM7.5 8.5l-3-3 1-1 3 3-1 1z"
        /> < title > { title } < / title > < / svg >
    }
}
