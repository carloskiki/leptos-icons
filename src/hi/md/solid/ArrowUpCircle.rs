#[cfg(feature = "HiMdSolidArrowUpCircle")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowUpCircle")]
///This icon requires the feature `HiMdSolidArrowUpCircle` to be enabled.
#[component]
pub fn ArrowUpCircle(
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
        "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM9.25 13.25C9.25 13.6642 9.58579 14 10 14C10.4142 14 10.75 13.6642 10.75 13.25V8.6599L12.7004 10.7603C12.9823 11.0639 13.4568 11.0814 13.7603 10.7996C14.0639 10.5177 14.0814 10.0432 13.7996 9.73966L10.5496 6.23966C10.4077 6.08684 10.2086 6 10 6C9.79145 6 9.59231 6.08684 9.45041 6.23966L6.20041 9.73966C5.91855 10.0432 5.93613 10.5177 6.23966 10.7996C6.54319 11.0814 7.01774 11.0639 7.2996 10.7603L9.25 8.6599V13.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
