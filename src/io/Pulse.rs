#[cfg(feature = "IoPulse")]
use leptos::*;
#[cfg(feature = "IoPulse")]
///This icon requires the feature `IoPulse` to be enabled.
#[component]
pub fn Pulse(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M432,272a48.09,48.09,0,0,0-45.25,32H347.53l-28.35-85.06a16,16,0,0,0-30.56.66L244.11,375.36l-52.33-314a16,16,0,0,0-31.3-1.25L99.51,304H48a16,16,0,0,0,0,32h64a16,16,0,0,0,15.52-12.12l45.34-181.37,51.36,308.12A16,16,0,0,0,239.1,464c.3,0,.6,0,.91,0a16,16,0,0,0,15.37-11.6l49.8-174.28,15.64,46.94A16,16,0,0,0,336,336h50.75A48,48,0,1,0,432,272Z"
        /> < title > { title } < / title > < / svg >
    }
}
