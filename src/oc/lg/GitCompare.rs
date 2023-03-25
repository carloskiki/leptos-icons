#[cfg(feature = "OcLgGitCompare")]
use leptos::*;
#[cfg(feature = "OcLgGitCompare")]
///This icon requires the feature `OcLgGitCompare` to be enabled.
#[component]
pub fn GitCompare(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M16.5 19.25a3.25 3.25 0 1 1 6.5 0 3.25 3.25 0 0 1-6.5 0Zm3.25-1.75a1.75 1.75 0 1 0 .001 3.501 1.75 1.75 0 0 0-.001-3.501Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13.905 1.72a.75.75 0 0 1 0 1.06L12.685 4h4.065a3.75 3.75 0 0 1 3.75 3.75v8.75a.75.75 0 0 1-1.5 0V7.75a2.25 2.25 0 0 0-2.25-2.25h-4.064l1.22 1.22a.75.75 0 0 1-1.061 1.06l-2.5-2.5a.75.75 0 0 1 0-1.06l2.5-2.5a.75.75 0 0 1 1.06 0ZM7.5 4.75a3.25 3.25 0 1 1-6.5 0 3.25 3.25 0 0 1 6.5 0ZM4.25 6.5a1.75 1.75 0 1 0-.001-3.501A1.75 1.75 0 0 0 4.25 6.5Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.095 22.28a.75.75 0 0 1 0-1.06l1.22-1.22H7.25a3.75 3.75 0 0 1-3.75-3.75V7.5a.75.75 0 0 1 1.5 0v8.75a2.25 2.25 0 0 0 2.25 2.25h4.064l-1.22-1.22a.748.748 0 0 1 .332-1.265.75.75 0 0 1 .729.205l2.5 2.5a.75.75 0 0 1 0 1.06l-2.5 2.5a.75.75 0 0 1-1.06 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
