#[cfg(feature = "HiMdSolidArrowLongUp")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowLongUp")]
///This icon requires the feature `HiMdSolidArrowLongUp` to be enabled.
#[component]
pub fn ArrowLongUp(
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
        "M10 18C9.58578 18 9.25 17.6642 9.25 17.25L9.25 4.6599L7.29959 6.76034C7.01774 7.06387 6.54319 7.08145 6.23966 6.7996C5.93612 6.51774 5.91855 6.0432 6.2004 5.73966L9.4504 2.23966C9.59231 2.08684 9.79144 2 10 2C10.2085 2 10.4077 2.08684 10.5496 2.23966L13.7996 5.73966C14.0814 6.04319 14.0639 6.51774 13.7603 6.79959C13.4568 7.08145 12.9823 7.06387 12.7004 6.76034L10.75 4.6599L10.75 17.25C10.75 17.6642 10.4142 18 10 18Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
