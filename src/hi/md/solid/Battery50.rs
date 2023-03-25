#[cfg(feature = "HiMdSolidBattery50")]
use leptos::*;
#[cfg(feature = "HiMdSolidBattery50")]
///This icon requires the feature `HiMdSolidBattery50` to be enabled.
#[component]
pub fn Battery50(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.75 8C4.33579 8 4 8.33579 4 8.75V11.25C4 11.6642 4.33579 12 4.75 12H9.5C9.91421 12 10.25 11.6642 10.25 11.25V8.75C10.25 8.33579 9.91421 8 9.5 8H4.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M3.25 5C2.00736 5 1 6.00736 1 7.25V12.75C1 13.9926 2.00736 15 3.25 15H15.75C16.9926 15 18 13.9926 18 12.75V11.6646C18.5826 11.4587 19 10.9031 19 10.25V9.75C19 9.09689 18.5826 8.54127 18 8.33535V7.25C18 6.00736 16.9926 5 15.75 5H3.25ZM2.5 7.25C2.5 6.83579 2.83579 6.5 3.25 6.5H15.75C16.1642 6.5 16.5 6.83579 16.5 7.25V12.75C16.5 13.1642 16.1642 13.5 15.75 13.5H3.25C2.83579 13.5 2.5 13.1642 2.5 12.75V7.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
