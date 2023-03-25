#[cfg(feature = "VsDebugLineByLine")]
use leptos::*;
#[cfg(feature = "VsDebugLineByLine")]
///This icon requires the feature `VsDebugLineByLine` to be enabled.
#[component]
pub fn DebugLineByLine(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 10V9h9v1H6zm4-4h5v1h-5V6zm5-3v1H6V3h9zm-9 9v1h9v-1H6z" />< path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M1 2.795l.783-.419 5.371 3.581v.838l-5.371 3.581L1 9.957V2.795zm1.007.94v5.281l3.96-2.64-3.96-2.64z"
        /> < title > { title } < / title > < / svg >
    }
}
