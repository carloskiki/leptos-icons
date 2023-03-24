#[cfg(feature = "HiMdSolidMap")]
use leptos::*;
#[cfg(feature = "HiMdSolidMap")]
///This icon requires the feature `HiMdSolidMap` to be enabled.
#[component]
pub fn Map(
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
        "M8.15686 2.1755C7.78961 2.02353 7.37706 2.02353 7.00981 2.1755L2.92647 3.86515C2.3657 4.0972 2 4.64429 2 5.25118V16.1278C2 17.1966 3.08594 17.9225 4.07353 17.5139L7.58333 16.0615L11.8431 17.8242C12.2104 17.9762 12.6229 17.9762 12.9902 17.8242L17.0735 16.1345C17.6343 15.9025 18 15.3554 18 14.7485V3.87187C18 2.80308 16.9141 2.07719 15.9265 2.48584L12.4167 3.93818L8.15686 2.1755ZM7.58008 4.99988C7.99429 4.99988 8.33008 5.33566 8.33008 5.74988V12.2499C8.33008 12.6641 7.99429 12.9999 7.58008 12.9999C7.16586 12.9999 6.83008 12.6641 6.83008 12.2499V5.74988C6.83008 5.33566 7.16586 4.99988 7.58008 4.99988ZM13.1699 7.74988C13.1699 7.33566 12.8341 6.99988 12.4199 6.99988C12.0057 6.99988 11.6699 7.33566 11.6699 7.74988V14.2499C11.6699 14.6641 12.0057 14.9999 12.4199 14.9999C12.8341 14.9999 13.1699 14.6641 13.1699 14.2499V7.74988Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
