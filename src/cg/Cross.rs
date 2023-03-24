#[cfg(feature = "CgCross")]
use leptos::*;
#[cfg(feature = "CgCross")]
///This icon requires the feature `CgCross` to be enabled.
#[component]
pub fn Cross(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M13 6C13 5.44772 12.5523 5 12 5C11.4477 5 11 5.44772 11 6V9H7C6.44772 9 6 9.44772 6 10C6 10.5523 6.44772 11 7 11H11V18C11 18.5523 11.4477 19 12 19C12.5523 19 13 18.5523 13 18V11H17C17.5523 11 18 10.5523 18 10C18 9.44772 17.5523 9 17 9H13V6Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
