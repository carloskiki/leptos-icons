#[cfg(feature = "HiLgSolidWallet")]
use leptos::*;
#[cfg(feature = "HiLgSolidWallet")]
///This icon requires the feature `HiLgSolidWallet` to be enabled.
#[component]
pub fn Wallet(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M2.27307 5.62524C3.06638 4.92494 4.10851 4.5 5.24989 4.5H18.7499C19.8913 4.5 20.9334 4.92494 21.7267 5.62524C21.5423 4.14526 20.2798 3 18.7499 3H5.24989C3.71995 3 2.4575 4.14525 2.27307 5.62524Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2.27307 8.62524C3.06638 7.92494 4.10851 7.5 5.24989 7.5H18.7499C19.8913 7.5 20.9334 7.92494 21.7267 8.62524C21.5423 7.14526 20.2798 6 18.7499 6H5.24989C3.71995 6 2.4575 7.14525 2.27307 8.62524Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.25 9C3.59315 9 2.25 10.3431 2.25 12V18C2.25 19.6569 3.59315 21 5.25 21H18.75C20.4069 21 21.75 19.6569 21.75 18V12C21.75 10.3431 20.4069 9 18.75 9H15C14.5858 9 14.25 9.33579 14.25 9.75C14.25 10.9926 13.2426 12 12 12C10.7574 12 9.75 10.9926 9.75 9.75C9.75 9.33579 9.41421 9 9 9H5.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
