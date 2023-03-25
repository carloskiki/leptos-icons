#[cfg(feature = "RiLogosFillTwitch")]
use leptos::*;
#[cfg(feature = "RiLogosFillTwitch")]
///This icon requires the feature `RiLogosFillTwitch` to be enabled.
#[component]
pub fn Twitch(
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
        "M21 3v11.74l-4.696 4.695h-3.913l-2.437 2.348H6.913v-2.348H3V6.13L4.227 3H21zm-1.565 1.565H6.13v11.74h3.13v2.347l2.349-2.348h4.695l3.13-3.13V4.565zm-3.13 3.13v4.696h-1.566V7.696h1.565zm-3.914 0v4.696h-1.565V7.696h1.565z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
