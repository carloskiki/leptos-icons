#[cfg(feature = "VsDebugAltSmall")]
use leptos::*;
#[cfg(feature = "VsDebugAltSmall")]
///This icon requires the feature `VsDebugAltSmall` to be enabled.
#[component]
pub fn DebugAltSmall(
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
        "M7.293 9.006l-.88.88A2.484 2.484 0 0 0 4 8a2.488 2.488 0 0 0-2.413 1.886l-.88-.88L0 9.712l1.147 1.146-.147.146v1H0v.999h1v.053c.051.326.143.643.273.946L0 15.294.707 16l1.1-1.099A2.873 2.873 0 0 0 4 16a2.875 2.875 0 0 0 2.193-1.099L7.293 16 8 15.294l-1.273-1.292A3.92 3.92 0 0 0 7 13.036v-.067h1v-.965H7v-1l-.147-.146L8 9.712l-.707-.706zM4 9.006a1.5 1.5 0 0 1 1.5 1.499h-3A1.498 1.498 0 0 1 4 9.006zm2 3.997A2.217 2.217 0 0 1 4 15a2.22 2.22 0 0 1-2-1.998v-1.499h4v1.499z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule
        = "evenodd" d = "M5 2.41L5.78 2l9 6v.83L9 12.683v-1.2l4.6-3.063L6 3.35V7H5V2.41z"
        /> < title > { title } < / title > < / svg >
    }
}
