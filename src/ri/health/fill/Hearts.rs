#[cfg(feature = "RiHealthFillHearts")]
use leptos::*;
#[cfg(feature = "RiHealthFillHearts")]
///This icon requires the feature `RiHealthFillHearts` to be enabled.
#[component]
pub fn Hearts(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M17.363 11.045c1.404-1.393 3.68-1.393 5.084 0 1.404 1.394 1.404 3.654 0 5.047L17 21.5l-5.447-5.408c-1.404-1.393-1.404-3.653 0-5.047 1.404-1.393 3.68-1.393 5.084 0l.363.36.363-.36zm1.88-6.288c.94.943 1.503 2.118 1.689 3.338-1.333-.248-2.739-.01-3.932.713-2.15-1.303-4.994-1.03-6.856.818-2.131 2.115-2.19 5.515-.178 7.701l.178.185 2.421 2.404L11 21.485 2.52 12.993C.417 10.637.496 7.019 2.757 4.757c2.265-2.264 5.888-2.34 8.244-.228 2.349-2.109 5.979-2.039 8.242.228z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
