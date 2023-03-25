#[cfg(feature = "SiAol")]
use leptos::*;
#[cfg(feature = "SiAol")]
///This icon requires the feature `SiAol` to be enabled.
#[component]
pub fn Aol(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M13.07 9.334c2.526 0 3.74 1.997 3.74 3.706 0 1.709-1.214 3.706-3.74 3.706-2.527 0-3.74-1.997-3.74-3.706 0-1.709 1.213-3.706 3.74-3.706m0 5.465c.9 0 1.663-.741 1.663-1.759 0-1.018-.763-1.759-1.663-1.759s-1.664.741-1.664 1.759c0 1.018.764 1.76 1.664 1.76m4.913-7.546h2.104v9.298h-2.104zm4.618 6.567a1.398 1.398 0 1 0 .002 2.796 1.398 1.398 0 0 0-.002-2.796M5.536 7.254H3.662L0 16.55h2.482l.49-1.343h3.23l.452 1.343H9.16zm-1.91 6.068L4.6 10.08l.974 3.242H3.626z"
        /> < title > { title } < / title > < / svg >
    }
}
