#[cfg(feature = "BiSolidSushi")]
use leptos::*;
#[cfg(feature = "BiSolidSushi")]
///This icon requires the feature `BiSolidSushi` to be enabled.
#[component]
pub fn Sushi(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > <
        ellipse xmlns = "http://www.w3.org/2000/svg" cx = "12.07" cy = "7" rx = "3" ry =
        "1.71" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.07 22c4.48 0 8-2.2 8-5V7c0-2.8-3.52-5-8-5s-8 2.2-8 5v10c0 2.8 3.51 5 8 5zm0-18c3.53 0 6 1.58 6 3a2 2 0 0 1-.29.87c-.68 1-2.53 2-5 2.12h-1.39C8.88 9.83 7 8.89 6.35 7.84a2.16 2.16 0 0 1-.28-.76V7c0-1.42 2.46-3 6-3z"
        /> < title > { title } < / title > < / svg >
    }
}
