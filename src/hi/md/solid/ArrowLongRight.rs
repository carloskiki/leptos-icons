#[cfg(feature = "HiMdSolidArrowLongRight")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowLongRight")]
///This icon requires the feature `HiMdSolidArrowLongRight` to be enabled.
#[component]
pub fn ArrowLongRight(
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
        "M2 10C2 9.58579 2.33579 9.25 2.75 9.25L15.3401 9.25L13.2397 7.2996C12.9361 7.01775 12.9186 6.5432 13.2004 6.23966C13.4823 5.93613 13.9568 5.91856 14.2603 6.20041L17.7603 9.45041C17.9132 9.59232 18 9.79145 18 10C18 10.2086 17.9132 10.4077 17.7603 10.5496L14.2603 13.7996C13.9568 14.0815 13.4823 14.0639 13.2004 13.7603C12.9186 13.4568 12.9361 12.9823 13.2397 12.7004L15.3401 10.75L2.75 10.75C2.33579 10.75 2 10.4142 2 10Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
