#[cfg(feature = "SiZalando")]
use leptos::*;
#[cfg(feature = "SiZalando")]
///This icon requires the feature `SiZalando` to be enabled.
#[component]
pub fn Zalando(
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
        "M5.27 24c-.88 0-1.36-.2-1.62-.36-.36-.21-1.02-.75-1.62-2.33A27.06 27.06 0 01.49 12c.02-3.66.59-6.76 1.54-9.3C2.63 1.1 3.29.56 3.65.35 3.91.21 4.39 0 5.27 0c.33 0 .72.03 1.18.1a26.1 26.1 0 018.7 3.3h.01a26.4 26.4 0 017.16 6.01c1.06 1.32 1.19 2.17 1.19 2.59 0 .42-.13 1.27-1.19 2.59a26.4 26.4 0 01-7.16 6h-.01a26.03 26.03 0 01-8.7 3.3c-.46.08-.85.11-1.18.11z"
        /> < title > { title } < / title > < / svg >
    }
}
