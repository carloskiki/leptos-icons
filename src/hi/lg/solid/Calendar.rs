#[cfg(feature = "HiLgSolidCalendar")]
use leptos::*;
#[cfg(feature = "HiLgSolidCalendar")]
///This icon requires the feature `HiLgSolidCalendar` to be enabled.
#[component]
pub fn Calendar(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M6.75 2.25C7.16421 2.25 7.5 2.58579 7.5 3V4.5H16.5V3C16.5 2.58579 16.8358 2.25 17.25 2.25C17.6642 2.25 18 2.58579 18 3V4.5H18.75C20.4069 4.5 21.75 5.84315 21.75 7.5V18.75C21.75 20.4069 20.4069 21.75 18.75 21.75H5.25C3.59315 21.75 2.25 20.4069 2.25 18.75V7.5C2.25 5.84315 3.59315 4.5 5.25 4.5H6V3C6 2.58579 6.33579 2.25 6.75 2.25ZM20.25 11.25C20.25 10.4216 19.5784 9.75 18.75 9.75H5.25C4.42157 9.75 3.75 10.4216 3.75 11.25V18.75C3.75 19.5784 4.42157 20.25 5.25 20.25H18.75C19.5784 20.25 20.25 19.5784 20.25 18.75V11.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
