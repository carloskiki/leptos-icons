#[cfg(feature = "CgCornerRightDown")]
use leptos::*;
#[cfg(feature = "CgCornerRightDown")]
///This icon requires the feature `CgCornerRightDown` to be enabled.
#[component]
pub fn CornerRightDown(
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
        "M13.3987 6.84996C13.3977 5.74539 12.5014 4.85079 11.3969 4.85181L3.39687 4.8592L3.39502 2.8592L11.395 2.85181C13.6042 2.84977 15.3967 4.63897 15.3987 6.84811L15.4082 17.1538L19.1895 13.3656L20.605 14.7785L14.2469 21.1483L7.87709 14.7902L9.28999 13.3747L13.4085 17.4856L13.3987 6.84996Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
