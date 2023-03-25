#[cfg(feature = "CgRemote")]
use leptos::*;
#[cfg(feature = "CgRemote")]
///This icon requires the feature `CgRemote` to be enabled.
#[component]
pub fn Remote(
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
        "M17.0514 4.32178L18.4656 5.73599L14.223 9.97863L18.4656 14.2213L17.0514 15.6355L11.3946 9.97863L17.0514 4.32178Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.94864 19.6785L5.53442 18.2643L9.77706 14.0216L5.53442 9.77897L6.94864 8.36476L12.6055 14.0216L6.94864 19.6785Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
