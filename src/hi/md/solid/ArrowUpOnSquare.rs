#[cfg(feature = "HiMdSolidArrowUpOnSquare")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowUpOnSquare")]
///This icon requires the feature `HiMdSolidArrowUpOnSquare` to be enabled.
#[component]
pub fn ArrowUpOnSquare(
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
        "M13.75 7H10.75L10.75 3.6599L12.7004 5.76034C12.9823 6.06387 13.4568 6.08145 13.7603 5.7996C14.0639 5.51774 14.0814 5.04319 13.7996 4.73966L10.5496 1.23966C10.4077 1.08684 10.2085 1 10 1C9.79145 1 9.59231 1.08684 9.4504 1.23966L6.2004 4.73966C5.91855 5.04319 5.93613 5.51774 6.23966 5.79959C6.54319 6.08145 7.01774 6.06387 7.29959 5.76034L9.25 3.6599L9.25 7H6.25C5.00736 7 4 8.00736 4 9.25V16.75C4 17.9926 5.00736 19 6.25 19H13.75C14.9926 19 16 17.9926 16 16.75V9.25C16 8.00736 14.9926 7 13.75 7ZM10.75 7H9.25L9.25 12.25C9.25 12.6642 9.58579 13 10 13C10.4142 13 10.75 12.6642 10.75 12.25L10.75 7Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
