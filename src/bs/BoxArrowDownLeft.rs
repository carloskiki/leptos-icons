#[cfg(feature = "BsBoxArrowDownLeft")]
use leptos::*;
#[cfg(feature = "BsBoxArrowDownLeft")]
///This icon requires the feature `BsBoxArrowDownLeft` to be enabled.
#[component]
pub fn BoxArrowDownLeft(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-box-arrow-down-left" viewBox = "0 0 16 16" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" d =
        "M7.364 12.5a.5.5 0 0 0 .5.5H14.5a1.5 1.5 0 0 0 1.5-1.5v-10A1.5 1.5 0 0 0 14.5 0h-10A1.5 1.5 0 0 0 3 1.5v6.636a.5.5 0 1 0 1 0V1.5a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 .5.5v10a.5.5 0 0 1-.5.5H7.864a.5.5 0 0 0-.5.5z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill - rule = "evenodd" d =
        "M0 15.5a.5.5 0 0 0 .5.5h5a.5.5 0 0 0 0-1H1.707l8.147-8.146a.5.5 0 0 0-.708-.708L1 14.293V10.5a.5.5 0 0 0-1 0v5z"
        /> < title > { title } < / title > < / svg >
    }
}
