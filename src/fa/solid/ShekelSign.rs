#[cfg(feature = "FaSolidShekelSign")]
use leptos::*;
#[cfg(feature = "FaSolidShekelSign")]
///This icon requires the feature `FaSolidShekelSign` to be enabled.
#[component]
pub fn ShekelSign(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M32 32C14.3 32 0 46.3 0 64V448c0 17.7 14.3 32 32 32s32-14.3 32-32V96H192c35.3 0 64 28.7 64 64V320c0 17.7 14.3 32 32 32s32-14.3 32-32V160c0-70.7-57.3-128-128-128H32zM320 480c70.7 0 128-57.3 128-128V64c0-17.7-14.3-32-32-32s-32 14.3-32 32V352c0 35.3-28.7 64-64 64H192V192c0-17.7-14.3-32-32-32s-32 14.3-32 32V448c0 17.7 14.3 32 32 32H320z"
        /> < title > { title } < / title > < / svg >
    }
}
