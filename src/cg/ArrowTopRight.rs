#[cfg(feature = "CgArrowTopRight")]
use leptos::*;
#[cfg(feature = "CgArrowTopRight")]
///This icon requires the feature `CgArrowTopRight` to be enabled.
#[component]
pub fn ArrowTopRight(
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
        "M10.5253 5.49475L10.5206 7.49475L15.0782 7.50541L5.47473 17.0896L6.88752 18.5052L16.5173 8.89479L16.5065 13.5088L18.5065 13.5134L18.5253 5.51345L10.5253 5.49475Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
