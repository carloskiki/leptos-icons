#[cfg(feature = "FaBrandsQuinscape")]
use leptos::*;
#[cfg(feature = "FaBrandsQuinscape")]
///This icon requires the feature `FaBrandsQuinscape` to be enabled.
#[component]
pub fn Quinscape(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M313.6 474.6h-1a158.1 158.1 0 0 1 0-316.2c94.9 0 168.2 83.1 157 176.6 4 5.1 8.2 9.6 11.2 15.3 13.4-30.3 20.3-62.4 20.3-97.7C501.1 117.5 391.6 8 256.5 8S12 117.5 12 252.6s109.5 244.6 244.5 244.6a237.36 237.36 0 0 0 70.4-10.1c-5.2-3.5-8.9-8.1-13.3-12.5zm-.1-.1l.4.1zm78.4-168.9a99.2 99.2 0 1 0 99.2 99.2 99.18 99.18 0 0 0-99.2-99.2z"
        /> < title > { title } < / title > < / svg >
    }
}
