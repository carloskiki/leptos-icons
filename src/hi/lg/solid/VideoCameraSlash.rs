#[cfg(feature = "HiLgSolidVideoCameraSlash")]
use leptos::*;
#[cfg(feature = "HiLgSolidVideoCameraSlash")]
///This icon requires the feature `HiLgSolidVideoCameraSlash` to be enabled.
#[component]
pub fn VideoCameraSlash(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.53033 2.46967C3.23744 2.17678 2.76256 2.17678 2.46967 2.46967C2.17678 2.76256 2.17678 3.23744 2.46967 3.53033L20.4697 21.5303C20.7626 21.8232 21.2374 21.8232 21.5303 21.5303C21.8232 21.2374 21.8232 20.7626 21.5303 20.4697L3.53033 2.46967Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M22.5 17.6893C22.5 18.1614 22.2984 18.5502 21.996 18.814L17.25 14.068V7.93931L19.9393 5.24996C20.8843 4.30501 22.5 4.97427 22.5 6.31063V17.6893Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.75 7.5V12.568L7.68198 4.5H12.75C14.4069 4.5 15.75 5.84315 15.75 7.5Z" fill =
        "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M1.5 7.5C1.5 6.71787 1.79931 6.00564 2.28963 5.47161L15.1363 18.3183C14.5882 19.0366 13.7232 19.5 12.75 19.5H4.5C2.84315 19.5 1.5 18.1569 1.5 16.5V7.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
