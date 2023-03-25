#[cfg(feature = "HiLgSolidBookmarkSlash")]
use leptos::*;
#[cfg(feature = "HiLgSolidBookmarkSlash")]
///This icon requires the feature `HiLgSolidBookmarkSlash` to be enabled.
#[component]
pub fn BookmarkSlash(
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
        "M3.53033 2.46967C3.23744 2.17678 2.76256 2.17678 2.46967 2.46967C2.17678 2.76256 2.17678 3.23744 2.46967 3.53033L20.4697 21.5303C20.7626 21.8232 21.2374 21.8232 21.5303 21.5303C21.8232 21.2374 21.8232 20.7626 21.5303 20.4697L3.53033 2.46967Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.25 5.50699V17.068L5.85284 2.67086C6.00319 2.62762 6.15925 2.59609 6.32022 2.57741C8.18374 2.36114 10.079 2.25 12 2.25C13.921 2.25 15.8163 2.36114 17.6798 2.57741C19.1772 2.75119 20.25 4.03722 20.25 5.50699Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.75 21V6.93198L17.8131 20.9951L12 18.0885L4.83541 21.6708C4.60292 21.7871 4.32681 21.7746 4.1057 21.638C3.88459 21.5013 3.75 21.2599 3.75 21Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
