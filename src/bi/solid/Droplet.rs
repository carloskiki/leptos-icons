#[cfg(feature = "BiSolidDroplet")]
use leptos::*;
#[cfg(feature = "BiSolidDroplet")]
///This icon requires the feature `BiSolidDroplet` to be enabled.
#[component]
pub fn Droplet(
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
        "M12.6 2.4c-.4-.3-.9-.3-1.2 0C9.5 3.9 4 8.5 4 14c0 4.4 3.6 8 8 8s8-3.6 8-8c0-5.4-5.5-10.1-7.4-11.6"
        /> < title > { title } < / title > < / svg >
    }
}
