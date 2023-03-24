#[cfg(feature = "SiBuddy")]
use leptos::*;
#[cfg(feature = "SiBuddy")]
///This icon requires the feature `SiBuddy` to be enabled.
#[component]
pub fn Buddy(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M21.7 5.307L12.945.253a1.892 1.892 0 00-1.891 0L2.299 5.306a1.892 1.892 0 00-.945 1.638v10.11c0 .675.36 1.3.945 1.637l8.756 5.056a1.892 1.892 0 001.89 0l8.756-5.055c.585-.338.945-.962.945-1.638V6.945c0-.675-.36-1.3-.945-1.638zm-7.45 7.753l-3.805 3.804-1.351-1.351 3.804-3.805-3.804-3.806 1.35-1.35 3.805 3.805 1.351 1.35-1.35 1.353z"
        /> < title > { title } < / title > < / svg >
    }
}
