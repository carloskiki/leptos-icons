#[cfg(feature = "BiRegularCast")]
use leptos::*;
#[cfg(feature = "BiRegularCast")]
///This icon requires the feature `BiRegularCast` to be enabled.
#[component]
pub fn Cast(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M11 21.001h2C13 14.935 8.065 10 2 10v2c4.962 0 9 4.038 9 9.001z" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M7 21.001h2C9 17.141 5.86 14 2 14v2c2.757 0 5 2.243 5 5.001z" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "3.5" cy = "19.5" r = "1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M20 4H4c-1.103 0-2 .897-2 2v2.052c.68.025 1.349.094 2 .217V6h16v13h-5.269c.123.651.191 1.32.217 2H20c1.103 0 2-.897 2-2V6c0-1.103-.897-2-2-2z"
        /> < title > { title } < / title > < / svg >
    }
}
