#[cfg(feature = "CgMergeHorizontal")]
use leptos::*;
#[cfg(feature = "CgMergeHorizontal")]
///This icon requires the feature `CgMergeHorizontal` to be enabled.
#[component]
pub fn MergeHorizontal(
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
        "M12 8.9757L16.2426 4.73306L14.8284 3.31885L12 6.14727L9.17157 3.31885L7.75736 4.73306L12 8.9757Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 11.9998C5 11.4475 5.44772 10.9997 6 10.9997H18C18.5523 10.9997 19 11.4475 19 11.9998C19 12.552 18.5523 12.9998 18 12.9998H6C5.44772 12.9998 5 12.552 5 11.9998Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 15.0242L7.75736 19.2668L9.17157 20.681L12 17.8526L14.8284 20.681L16.2426 19.2668L12 15.0242Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
