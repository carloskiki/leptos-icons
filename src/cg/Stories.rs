#[cfg(feature = "CgStories")]
use leptos::*;
#[cfg(feature = "CgStories")]
///This icon requires the feature `CgStories` to be enabled.
#[component]
pub fn Stories(
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
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M15 6H9C8.44772 6 8 6.44772 8 7V17C8 17.5523 8.44772 18 9 18H15C15.5523 18 16 17.5523 16 17V7C16 6.44772 15.5523 6 15 6ZM9 4C7.34315 4 6 5.34315 6 7V17C6 18.6569 7.34315 20 9 20H15C16.6569 20 18 18.6569 18 17V7C18 5.34315 16.6569 4 15 4H9Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 6C2 5.44772 2.44772 5 3 5C3.55228 5 4 5.44772 4 6V18C4 18.5523 3.55228 19 3 19C2.44772 19 2 18.5523 2 18V6Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 6C20 5.44772 20.4477 5 21 5C21.5523 5 22 5.44772 22 6V18C22 18.5523 21.5523 19 21 19C20.4477 19 20 18.5523 20 18V6Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
