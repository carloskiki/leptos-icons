#[cfg(feature = "HiMdSolidArrowDownTray")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowDownTray")]
///This icon requires the feature `HiMdSolidArrowDownTray` to be enabled.
#[component]
pub fn ArrowDownTray(
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
        "M10.75 2.75C10.75 2.33579 10.4142 2 10 2C9.58579 2 9.25 2.33579 9.25 2.75V11.3636L6.29526 8.23503C6.01085 7.93389 5.53617 7.92033 5.23503 8.20474C4.9339 8.48915 4.92033 8.96383 5.20474 9.26497L9.45474 13.765C9.59642 13.915 9.79366 14 10 14C10.2063 14 10.4036 13.915 10.5453 13.765L14.7953 9.26497C15.0797 8.96383 15.0661 8.48915 14.765 8.20474C14.4638 7.92033 13.9892 7.93389 13.7047 8.23503L10.75 11.3636V2.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.5 12.75C3.5 12.3358 3.16421 12 2.75 12C2.33579 12 2 12.3358 2 12.75V15.25C2 16.7688 3.23122 18 4.75 18H15.25C16.7688 18 18 16.7688 18 15.25V12.75C18 12.3358 17.6642 12 17.25 12C16.8358 12 16.5 12.3358 16.5 12.75V15.25C16.5 15.9404 15.9404 16.5 15.25 16.5H4.75C4.05964 16.5 3.5 15.9404 3.5 15.25V12.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
