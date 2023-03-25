#[cfg(feature = "OcSmDuplicate")]
use leptos::*;
#[cfg(feature = "OcSmDuplicate")]
///This icon requires the feature `OcSmDuplicate` to be enabled.
#[component]
pub fn Duplicate(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M10.5 3a.75.75 0 0 1 .75.75v1h1a.75.75 0 0 1 0 1.5h-1v1a.75.75 0 0 1-1.5 0v-1h-1a.75.75 0 0 1 0-1.5h1v-1A.75.75 0 0 1 10.5 3Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.75 0h7.5C15.216 0 16 .784 16 1.75v7.5A1.75 1.75 0 0 1 14.25 11h-7.5A1.75 1.75 0 0 1 5 9.25v-7.5C5 .784 5.784 0 6.75 0ZM6.5 1.75v7.5c0 .138.112.25.25.25h7.5a.25.25 0 0 0 .25-.25v-7.5a.25.25 0 0 0-.25-.25h-7.5a.25.25 0 0 0-.25.25Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M1.75 5A1.75 1.75 0 0 0 0 6.75v7.5C0 15.216.784 16 1.75 16h7.5A1.75 1.75 0 0 0 11 14.25v-1.5a.75.75 0 0 0-1.5 0v1.5a.25.25 0 0 1-.25.25h-7.5a.25.25 0 0 1-.25-.25v-7.5a.25.25 0 0 1 .25-.25h1.5a.75.75 0 0 0 0-1.5h-1.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
