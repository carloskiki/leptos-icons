#[cfg(feature = "CgSortZa")]
use leptos::*;
#[cfg(feature = "CgSortZa")]
///This icon requires the feature `CgSortZa` to be enabled.
#[component]
pub fn SortZa(
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
        "M6 16C6 16.5523 6.44772 17 7 17H17C17.5523 17 18 16.5523 18 16C18 15.4477 17.5523 15 17 15H7C6.44772 15 6 15.4477 6 16Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 12C8 12.5523 8.44772 13 9 13H15C15.5523 13 16 12.5523 16 12C16 11.4477 15.5523 11 15 11H9C8.44772 11 8 11.4477 8 12Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 9C10.4477 9 10 8.55229 10 8C10 7.44771 10.4477 7 11 7H13C13.5523 7 14 7.44771 14 8C14 8.55229 13.5523 9 13 9H11Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
