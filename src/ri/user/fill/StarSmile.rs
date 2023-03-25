#[cfg(feature = "RiUserFillStarSmile")]
use leptos::*;
#[cfg(feature = "RiUserFillStarSmile")]
///This icon requires the feature `RiUserFillStarSmile` to be enabled.
#[component]
pub fn StarSmile(
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
        "M12 .5l4.226 6.183 7.187 2.109-4.575 5.93.215 7.486L12 19.69l-7.053 2.518.215-7.486-4.575-5.93 7.187-2.109L12 .5zM10 12H8a4 4 0 0 0 7.995.2L16 12h-2a2 2 0 0 1-3.995.15L10 12z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
