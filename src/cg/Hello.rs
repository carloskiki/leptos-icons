#[cfg(feature = "CgHello")]
use leptos::*;
#[cfg(feature = "CgHello")]
///This icon requires the feature `CgHello` to be enabled.
#[component]
pub fn Hello(
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
        "M17.5 12C17.5 13.576 16.8371 14.9972 15.7749 16C14.7899 16.9299 13.4615 17.5 12 17.5C10.5385 17.5 9.21007 16.9299 8.22506 16C7.16289 14.9972 6.5 13.576 6.5 12H17.5Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M1 7C1 3.68629 3.68629 1 7 1H17C20.3137 1 23 3.68629 23 7V17C23 20.3137 20.3137 23 17 23H1V7ZM3.75 12C3.75 7.44365 7.44365 3.75 12 3.75C16.5563 3.75 20.25 7.44365 20.25 12C20.25 16.5563 16.5563 20.25 12 20.25C7.44365 20.25 3.75 16.5563 3.75 12Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
