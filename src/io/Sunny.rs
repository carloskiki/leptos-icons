#[cfg(feature = "IoSunny")]
use leptos::*;
#[cfg(feature = "IoSunny")]
///This icon requires the feature `IoSunny` to be enabled.
#[component]
pub fn Sunny(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M256,118a22,22,0,0,1-22-22V48a22,22,0,0,1,44,0V96A22,22,0,0,1,256,118Z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M256,486a22,22,0,0,1-22-22V416a22,22,0,0,1,44,0v48A22,22,0,0,1,256,486Z" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M369.14,164.86a22,22,0,0,1-15.56-37.55l33.94-33.94a22,22,0,0,1,31.11,31.11l-33.94,33.94A21.93,21.93,0,0,1,369.14,164.86Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M108.92,425.08a22,22,0,0,1-15.55-37.56l33.94-33.94a22,22,0,1,1,31.11,31.11l-33.94,33.94A21.94,21.94,0,0,1,108.92,425.08Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M464,278H416a22,22,0,0,1,0-44h48a22,22,0,0,1,0,44Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M96,278H48a22,22,0,0,1,0-44H96a22,22,0,0,1,0,44Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M403.08,425.08a21.94,21.94,0,0,1-15.56-6.45l-33.94-33.94a22,22,0,0,1,31.11-31.11l33.94,33.94a22,22,0,0,1-15.55,37.56Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M142.86,164.86a21.89,21.89,0,0,1-15.55-6.44L93.37,124.48a22,22,0,0,1,31.11-31.11l33.94,33.94a22,22,0,0,1-15.56,37.55Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,358A102,102,0,1,1,358,256,102.12,102.12,0,0,1,256,358Z" /> < title > {
        title } < / title > < / svg >
    }
}
