#[cfg(feature = "CgPocket")]
use leptos::*;
#[cfg(feature = "CgPocket")]
///This icon requires the feature `CgPocket` to be enabled.
#[component]
pub fn Pocket(
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
        "M3 4H21V11C21 15.9706 16.9706 20 12 20C7.02944 20 3 15.9706 3 11V4ZM1 2H23V11C23 17.0751 18.0751 22 12 22C5.92487 22 1 17.0751 1 11V2ZM11.2929 14.6935C11.6834 15.084 12.3166 15.084 12.7071 14.6935L16.9497 10.4508C17.3403 10.0603 17.3403 9.42714 16.9497 9.03661C16.5592 8.64609 15.9261 8.64609 15.5355 9.03661L12 12.5721L8.46447 9.03661C8.07394 8.64609 7.44078 8.64609 7.05025 9.03661C6.65973 9.42714 6.65973 10.0603 7.05025 10.4508L11.2929 14.6935Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
