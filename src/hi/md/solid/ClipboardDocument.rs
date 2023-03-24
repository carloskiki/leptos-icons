#[cfg(feature = "HiMdSolidClipboardDocument")]
use leptos::*;
#[cfg(feature = "HiMdSolidClipboardDocument")]
///This icon requires the feature `HiMdSolidClipboardDocument` to be enabled.
#[component]
pub fn ClipboardDocument(
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
        "M15.9877 3.0124C17.1186 3.1311 18.0001 4.08761 18.0001 5.25V11.75C18.0001 12.9926 16.9927 14 15.7501 14H13.5001V10.6213C13.5001 9.82567 13.184 9.06261 12.6214 8.5L9.50008 5.37868C9.1106 4.9892 8.62505 4.71787 8.09912 4.58776C8.35944 3.74123 9.10578 3.10756 10.0125 3.0124C10.1312 1.88145 11.0877 1 12.2501 1H13.7501C14.9125 1 15.869 1.88145 15.9877 3.0124ZM11.5001 3.25C11.5001 2.83579 11.8359 2.5 12.2501 2.5H13.7501C14.1643 2.5 14.5001 2.83579 14.5001 3.25V3.5H11.5001V3.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.5 6C2.67157 6 2 6.67157 2 7.5V16.5C2 17.3284 2.67157 18 3.5 18H10.5C11.3284 18 12 17.3284 12 16.5V10.6213C12 10.2235 11.842 9.84196 11.5607 9.56066L8.43934 6.43934C8.15804 6.15804 7.7765 6 7.37868 6H3.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
