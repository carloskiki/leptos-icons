#[cfg(feature = "ImInfinite")]
use leptos::*;
#[cfg(feature = "ImInfinite")]
///This icon requires the feature `ImInfinite` to be enabled.
#[component]
pub fn Infinite(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M12.25 11.75c-1.002 0-1.943-0.39-2.652-1.098l-1.598-1.598-1.598 1.598c-0.708 0.708-1.65 1.098-2.652 1.098s-1.944-0.39-2.652-1.098c-0.708-0.708-1.098-1.65-1.098-2.652s0.39-1.943 1.098-2.652c0.708-0.708 1.65-1.098 2.652-1.098s1.943 0.39 2.652 1.098l1.598 1.598 1.598-1.598c0.708-0.708 1.65-1.098 2.652-1.098s1.944 0.39 2.652 1.098c0.708 0.708 1.098 1.65 1.098 2.652s-0.39 1.943-1.098 2.652c-0.708 0.708-1.65 1.098-2.652 1.098zM10.652 9.598c0.427 0.427 0.994 0.662 1.598 0.662s1.171-0.235 1.598-0.662c0.427-0.427 0.662-0.994 0.662-1.598s-0.235-1.171-0.662-1.598c-0.427-0.427-0.994-0.662-1.598-0.662s-1.171 0.235-1.598 0.662l-1.598 1.598 1.598 1.598zM3.75 5.74c-0.604 0-1.171 0.235-1.598 0.662s-0.662 0.994-0.662 1.598c0 0.604 0.235 1.171 0.662 1.598s0.994 0.662 1.598 0.662c0.604 0 1.171-0.235 1.598-0.662l1.598-1.598-1.598-1.598c-0.427-0.427-0.994-0.662-1.598-0.662v0z"
        /> < title > { title } < / title > < / svg >
    }
}
