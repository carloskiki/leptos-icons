#[cfg(feature = "IoKeypad")]
use leptos::*;
#[cfg(feature = "IoKeypad")]
///This icon requires the feature `IoKeypad` to be enabled.
#[component]
pub fn Keypad(
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
        "http://www.w3.org/2000/svg" d = "M256,400a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,272a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M256,144a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,16a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M384,272a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M384,144a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M384,16a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M128,272a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M128,144a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M128,16a48,48,0,1,0,48,48,48,48,0,0,0-48-48Z" /> < title > { title } < / title >
        < / svg >
    }
}
