#[cfg(feature = "IoBowlingBall")]
use leptos::*;
#[cfg(feature = "IoBowlingBall")]
///This icon requires the feature `IoBowlingBall` to be enabled.
#[component]
pub fn BowlingBall(
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
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M414.39,97.61A224,224,0,1,0,97.61,414.39,224,224,0,1,0,414.39,97.61ZM288,224a24,24,0,1,1,24-24A24,24,0,0,1,288,224Zm8-72a24,24,0,1,1,24-24A24,24,0,0,1,296,152Zm64,40a24,24,0,1,1,24-24A24,24,0,0,1,360,192Z"
        /> < title > { title } < / title > < / svg >
    }
}
