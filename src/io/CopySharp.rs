#[cfg(feature = "IoCopySharp")]
use leptos::*;
#[cfg(feature = "IoCopySharp")]
///This icon requires the feature `IoCopySharp` to be enabled.
#[component]
pub fn CopySharp(
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
        "M456,480H136a24,24,0,0,1-24-24V128a16,16,0,0,1,16-16H456a24,24,0,0,1,24,24V456A24,24,0,0,1,456,480Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M112,80H400V56a24,24,0,0,0-24-24H60A28,28,0,0,0,32,60V376a24,24,0,0,0,24,24H80V112A32,32,0,0,1,112,80Z"
        /> < title > { title } < / title > < / svg >
    }
}
