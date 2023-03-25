#[cfg(feature = "CgArrowsScrollV")]
use leptos::*;
#[cfg(feature = "CgArrowsScrollV")]
///This icon requires the feature `CgArrowsScrollV` to be enabled.
#[component]
pub fn ArrowsScrollV(
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
        "M13.4142 10.1091L13.4179 10.1128L12.0037 11.527L12 11.5233L11.9963 11.527L10.5821 10.1128L10.5858 10.1091L7.76105 7.28433L9.17526 5.87012L12 8.69486L14.8247 5.87012L16.239 7.28433L13.4142 10.1091Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 15.3052L14.8247 18.1299L16.239 16.7157L13.4142 13.891L13.4179 13.8873L12.0037 12.4731L12 12.4767L11.9963 12.4731L10.5821 13.8873L10.5858 13.891L7.76105 16.7157L9.17526 18.1299L12 15.3052Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
