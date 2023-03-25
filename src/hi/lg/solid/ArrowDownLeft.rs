#[cfg(feature = "HiLgSolidArrowDownLeft")]
use leptos::*;
#[cfg(feature = "HiLgSolidArrowDownLeft")]
///This icon requires the feature `HiLgSolidArrowDownLeft` to be enabled.
#[component]
pub fn ArrowDownLeft(
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
        "M20.0303 3.96967C20.3232 4.26256 20.3232 4.73744 20.0303 5.03033L6.31066 18.75L15.75 18.75C16.1642 18.75 16.5 19.0858 16.5 19.5C16.5 19.9142 16.1642 20.25 15.75 20.25L4.5 20.25C4.08579 20.25 3.75 19.9142 3.75 19.5L3.75 8.25C3.75 7.83579 4.08579 7.5 4.5 7.5C4.91421 7.5 5.25 7.83579 5.25 8.25L5.25 17.6893L18.9697 3.96967C19.2626 3.67678 19.7374 3.67678 20.0303 3.96967Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
