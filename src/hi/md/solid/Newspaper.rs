#[cfg(feature = "HiMdSolidNewspaper")]
use leptos::*;
#[cfg(feature = "HiMdSolidNewspaper")]
///This icon requires the feature `HiMdSolidNewspaper` to be enabled.
#[component]
pub fn Newspaper(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M2 3.5C2 2.67157 2.67157 2 3.5 2H12.5C13.3284 2 14 2.67157 14 3.5V15.25C14 16.7688 15.2312 18 16.75 18H4.75C3.23122 18 2 16.7688 2 15.25V3.5ZM5.75 10.5C5.33579 10.5 5 10.8358 5 11.25C5 11.6642 5.33579 12 5.75 12H10.25C10.6642 12 11 11.6642 11 11.25C11 10.8358 10.6642 10.5 10.25 10.5H5.75ZM5.75 13.5C5.33579 13.5 5 13.8358 5 14.25C5 14.6642 5.33579 15 5.75 15H10.25C10.6642 15 11 14.6642 11 14.25C11 13.8358 10.6642 13.5 10.25 13.5H5.75ZM5 5.75C5 5.33579 5.33579 5 5.75 5H10.25C10.6642 5 11 5.33579 11 5.75V8.25C11 8.66421 10.6642 9 10.25 9H5.75C5.33579 9 5 8.66421 5 8.25V5.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.5 6.5H15.5V15.25C15.5 15.9404 16.0596 16.5 16.75 16.5C17.4404 16.5 18 15.9404 18 15.25V8C18 7.17157 17.3284 6.5 16.5 6.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
