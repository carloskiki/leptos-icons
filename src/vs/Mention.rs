#[cfg(feature = "VsMention")]
use leptos::*;
#[cfg(feature = "VsMention")]
///This icon requires the feature `VsMention` to be enabled.
#[component]
pub fn Mention(
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
        "M10.465 12.36a5.516 5.516 0 0 1-3.053.736 4.262 4.262 0 0 1-4.57-4.543 5.381 5.381 0 0 1 5.391-5.571c2.377 0 4.413 1.375 4.413 4.006 0 2.182-1.292 3.66-2.9 3.66-.676 0-1.1-.274-1.126-.917a2.012 2.012 0 0 1-1.756.913c-.969 0-1.629-.645-1.629-1.923 0-1.763 1.148-3.4 2.62-3.4a1.314 1.314 0 0 1 1.427.93l.211-.809h.9L9.6 8.646c-.226.916-.13 1.215.342 1.215.984 0 1.833-1.21 1.833-2.825 0-2.068-1.445-3.265-3.61-3.265-2.643 0-4.374 2.132-4.382 4.786a3.443 3.443 0 0 0 3.686 3.717c.973.04 1.94-.179 2.8-.634l.196.72zM6.217 8.639c0 .788.307 1.206.913 1.206.758 0 1.38-.6 1.683-1.831C9.136 6.746 8.85 6.1 7.94 6.1c-1.04 0-1.723 1.339-1.723 2.539z"
        /> < title > { title } < / title > < / svg >
    }
}
