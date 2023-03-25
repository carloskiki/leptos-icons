#[cfg(feature = "HiLgSolidChevronDoubleLeft")]
use leptos::*;
#[cfg(feature = "HiLgSolidChevronDoubleLeft")]
///This icon requires the feature `HiLgSolidChevronDoubleLeft` to be enabled.
#[component]
pub fn ChevronDoubleLeft(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M13.2803 3.96967C13.5732 4.26256 13.5732 4.73744 13.2803 5.03033L6.31066 12L13.2803 18.9697C13.5732 19.2626 13.5732 19.7374 13.2803 20.0303C12.9874 20.3232 12.5126 20.3232 12.2197 20.0303L4.71967 12.5303C4.42678 12.2374 4.42678 11.7626 4.71967 11.4697L12.2197 3.96967C12.5126 3.67678 12.9874 3.67678 13.2803 3.96967ZM19.2803 3.96967C19.5732 4.26256 19.5732 4.73744 19.2803 5.03033L12.3107 12L19.2803 18.9697C19.5732 19.2626 19.5732 19.7374 19.2803 20.0303C18.9874 20.3232 18.5126 20.3232 18.2197 20.0303L10.7197 12.5303C10.4268 12.2374 10.4268 11.7626 10.7197 11.4697L18.2197 3.96967C18.5126 3.67678 18.9874 3.67678 19.2803 3.96967Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
