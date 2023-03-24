#[cfg(feature = "HiMdSolidBuildingLibrary")]
use leptos::*;
#[cfg(feature = "HiMdSolidBuildingLibrary")]
///This icon requires the feature `HiMdSolidBuildingLibrary` to be enabled.
#[component]
pub fn BuildingLibrary(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M9.67411 2.07459C9.88011 1.97514 10.1202 1.97514 10.3262 2.07459L17.5762 5.57459C17.9493 5.75467 18.1057 6.20304 17.9256 6.57606C17.7576 6.92403 17.3561 7.08351 17.0002 6.95718V16.5H17.2502C17.6644 16.5 18.0002 16.8358 18.0002 17.25C18.0002 17.6642 17.6644 18 17.2502 18H2.75017C2.33596 18 2.00017 17.6642 2.00017 17.25C2.00017 16.8358 2.33596 16.5 2.75017 16.5H3.00017V6.95718C2.6442 7.08351 2.24274 6.92403 2.07476 6.57606C1.89468 6.20304 2.05109 5.75467 2.42411 5.57459L9.67411 2.07459ZM11 6C11 6.55228 10.5523 7 10 7C9.44772 7 9 6.55228 9 6C9 5.44772 9.44772 5 10 5C10.5523 5 11 5.44772 11 6ZM7.5 9.75C7.5 9.33579 7.16421 9 6.75 9C6.33579 9 6 9.33579 6 9.75V15.25C6 15.6642 6.33579 16 6.75 16C7.16421 16 7.5 15.6642 7.5 15.25V9.75ZM10.75 9.75C10.75 9.33579 10.4142 9 10 9C9.58579 9 9.25 9.33579 9.25 9.75V15.25C9.25 15.6642 9.58579 16 10 16C10.4142 16 10.75 15.6642 10.75 15.25V9.75ZM14 9.75C14 9.33579 13.6642 9 13.25 9C12.8358 9 12.5 9.33579 12.5 9.75V15.25C12.5 15.6642 12.8358 16 13.25 16C13.6642 16 14 15.6642 14 15.25V9.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
