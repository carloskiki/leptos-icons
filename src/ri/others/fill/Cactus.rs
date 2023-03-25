#[cfg(feature = "RiOthersFillCactus")]
use leptos::*;
#[cfg(feature = "RiOthersFillCactus")]
///This icon requires the feature `RiOthersFillCactus` to be enabled.
#[component]
pub fn Cactus(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M12 2c2.21 0 4 1.79 4 4v9h1c.55 0 1-.45 1-1V8c0-.552.448-1 1-1s1 .448 1 1v6c0 1.657-1.343 3-3 3h-1v3h2v2H6v-2h2v-6H7c-1.657 0-3-1.343-3-3V9c0-.552.448-1 1-1s1 .448 1 1v2c0 .55.45 1 1 1h1V6c0-2.21 1.79-4 4-4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
