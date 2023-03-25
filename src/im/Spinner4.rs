#[cfg(feature = "ImSpinner4")]
use leptos::*;
#[cfg(feature = "ImSpinner4")]
///This icon requires the feature `ImSpinner4` to be enabled.
#[component]
pub fn Spinner4(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M3 8c0-0.19 0.011-0.378 0.032-0.563l-2.89-0.939c-0.092 0.487-0.141 0.989-0.141 1.502 0 2.3 0.971 4.374 2.526 5.833l1.786-2.458c-0.814-0.889-1.312-2.074-1.312-3.375zM13 8c0 1.301-0.497 2.486-1.312 3.375l1.786 2.458c1.555-1.459 2.526-3.533 2.526-5.833 0-0.513-0.049-1.015-0.141-1.502l-2.89 0.939c0.021 0.185 0.032 0.373 0.032 0.563zM9 3.1c1.436 0.292 2.649 1.199 3.351 2.435l2.89-0.939c-1.144-2.428-3.473-4.188-6.241-4.534v3.038zM3.649 5.535c0.702-1.236 1.914-2.143 3.351-2.435v-3.038c-2.769 0.345-5.097 2.105-6.241 4.534l2.89 0.939zM10.071 12.552c-0.631 0.288-1.332 0.448-2.071 0.448s-1.44-0.16-2.071-0.448l-1.786 2.458c1.144 0.631 2.458 0.99 3.857 0.99s2.713-0.359 3.857-0.99l-1.786-2.458z"
        /> < title > { title } < / title > < / svg >
    }
}
