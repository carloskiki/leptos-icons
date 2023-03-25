#[cfg(feature = "IoBagRemove")]
use leptos::*;
#[cfg(feature = "IoBagRemove")]
///This icon requires the feature `IoBagRemove` to be enabled.
#[component]
pub fn BagRemove(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M454.66,169.4A31.86,31.86,0,0,0,432,160H368V144a112,112,0,0,0-224,0v16H80a32,32,0,0,0-32,32V408c0,39,33,72,72,72H392a72.22,72.22,0,0,0,50.48-20.55A69.48,69.48,0,0,0,464,409.25V192A31.78,31.78,0,0,0,454.66,169.4ZM320,336H192a16,16,0,0,1,0-32H320a16,16,0,0,1,0,32Zm16-176H176V144a80,80,0,0,1,160,0Z"
        /> < title > { title } < / title > < / svg >
    }
}
