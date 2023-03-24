#[cfg(feature = "IoPause")]
use leptos::*;
#[cfg(feature = "IoPause")]
///This icon requires the feature `IoPause` to be enabled.
#[component]
pub fn Pause(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M208,432H160a16,16,0,0,1-16-16V96a16,16,0,0,1,16-16h48a16,16,0,0,1,16,16V416A16,16,0,0,1,208,432Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M352,432H304a16,16,0,0,1-16-16V96a16,16,0,0,1,16-16h48a16,16,0,0,1,16,16V416A16,16,0,0,1,352,432Z"
        /> < title > { title } < / title > < / svg >
    }
}
