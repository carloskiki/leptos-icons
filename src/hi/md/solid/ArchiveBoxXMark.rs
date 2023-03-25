#[cfg(feature = "HiMdSolidArchiveBoxXMark")]
use leptos::*;
#[cfg(feature = "HiMdSolidArchiveBoxXMark")]
///This icon requires the feature `HiMdSolidArchiveBoxXMark` to be enabled.
#[component]
pub fn ArchiveBoxXMark(
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
        "M2 3C1.44772 3 1 3.44772 1 4V5C1 5.55228 1.44772 6 2 6H18C18.5523 6 19 5.55228 19 5V4C19 3.44772 18.5523 3 18 3H2Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M2 7.5H18L17.1885 15.2094C17.0813 16.2273 16.223 17 15.1995 17H4.80052C3.77701 17 2.91866 16.2273 2.81151 15.2094L2 7.5ZM7.21967 9.21967C7.51256 8.92678 7.98744 8.92678 8.28033 9.21967L10 10.9393L11.7197 9.21967C12.0126 8.92678 12.4874 8.92678 12.7803 9.21967C13.0732 9.51256 13.0732 9.98744 12.7803 10.2803L11.0607 12L12.7803 13.7197C13.0732 14.0126 13.0732 14.4874 12.7803 14.7803C12.4874 15.0732 12.0126 15.0732 11.7197 14.7803L10 13.0607L8.28033 14.7803C7.98744 15.0732 7.51256 15.0732 7.21967 14.7803C6.92678 14.4874 6.92678 14.0126 7.21967 13.7197L8.93934 12L7.21967 10.2803C6.92678 9.98744 6.92678 9.51256 7.21967 9.21967Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
