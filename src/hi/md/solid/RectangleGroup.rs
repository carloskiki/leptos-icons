#[cfg(feature = "HiMdSolidRectangleGroup")]
use leptos::*;
#[cfg(feature = "HiMdSolidRectangleGroup")]
///This icon requires the feature `HiMdSolidRectangleGroup` to be enabled.
#[component]
pub fn RectangleGroup(
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
        "M2.5 3C1.67157 3 1 3.67157 1 4.5V8.5C1 9.32843 1.67157 10 2.5 10H8.5C9.32843 10 10 9.32843 10 8.5V4.5C10 3.67157 9.32843 3 8.5 3H2.5ZM13.5 5C12.6716 5 12 5.67157 12 6.5V13.5C12 14.3284 12.6716 15 13.5 15H17.5C18.3284 15 19 14.3284 19 13.5V6.5C19 5.67157 18.3284 5 17.5 5H13.5ZM3.5 12C2.67157 12 2 12.6716 2 13.5V15.5C2 16.3284 2.67157 17 3.5 17H9.5C10.3284 17 11 16.3284 11 15.5V13.5C11 12.6716 10.3284 12 9.5 12H3.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
