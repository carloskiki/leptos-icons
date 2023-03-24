#[cfg(feature = "VsLockSmall")]
use leptos::*;
#[cfg(feature = "VsLockSmall")]
///This icon requires the feature `VsLockSmall` to be enabled.
#[component]
pub fn LockSmall(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 8L4 7H12L13 8V13L12 14H4L3 13V8ZM4 8V13H12V8H4Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M11 7V5C11 3.34315 9.65685 2 8 2C6.34315 2 5 3.34315 5 5V7H6V5C6 3.89543 6.89543 3 8 3C9.10457 3 10 3.89543 10 5V7H11Z"
        /> < title > { title } < / title > < / svg >
    }
}
