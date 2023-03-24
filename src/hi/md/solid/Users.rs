#[cfg(feature = "HiMdSolidUsers")]
use leptos::*;
#[cfg(feature = "HiMdSolidUsers")]
///This icon requires the feature `HiMdSolidUsers` to be enabled.
#[component]
pub fn Users(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 8C8.65685 8 10 6.65685 10 5C10 3.34315 8.65685 2 7 2C5.34315 2 4 3.34315 4 5C4 6.65685 5.34315 8 7 8Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.5 9C15.8807 9 17 7.88071 17 6.5C17 5.11929 15.8807 4 14.5 4C13.1193 4 12 5.11929 12 6.5C12 7.88071 13.1193 9 14.5 9Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M1.61528 16.428C1.21798 16.1736 0.987847 15.721 1.04605 15.2529C1.41416 12.292 3.93944 10 6.9999 10C10.0604 10 12.5856 12.2914 12.9537 15.2522C13.012 15.7203 12.7818 16.1729 12.3845 16.4273C10.8302 17.4225 8.98243 18 6.9999 18C5.01737 18 3.16959 17.4231 1.61528 16.428Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.5001 16C14.4647 16 14.4295 15.9998 14.3943 15.9993C14.4631 15.7025 14.4822 15.3885 14.4423 15.0671C14.2668 13.6562 13.7001 12.367 12.854 11.3116C13.3646 11.1105 13.9208 11 14.5028 11C16.4426 11 18.0956 12.2273 18.7279 13.9478C18.8638 14.3176 18.7045 14.7241 18.3671 14.9275C17.2379 15.6083 15.9147 16 14.5001 16Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
