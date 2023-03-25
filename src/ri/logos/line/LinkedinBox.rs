#[cfg(feature = "RiLogosLineLinkedinBox")]
use leptos::*;
#[cfg(feature = "RiLogosLineLinkedinBox")]
///This icon requires the feature `RiLogosLineLinkedinBox` to be enabled.
#[component]
pub fn LinkedinBox(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M4 3h16a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1zm1 2v14h14V5H5zm2.5 4a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zm-1 1h2v7.5h-2V10zm5.5.43c.584-.565 1.266-.93 2-.93 2.071 0 3.5 1.679 3.5 3.75v4.25h-2v-4.25a1.75 1.75 0 0 0-3.5 0v4.25h-2V10h2v.43z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
