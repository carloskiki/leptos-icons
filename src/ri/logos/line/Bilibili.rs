#[cfg(feature = "RiLogosLineBilibili")]
use leptos::*;
#[cfg(feature = "RiLogosLineBilibili")]
///This icon requires the feature `RiLogosLineBilibili` to be enabled.
#[component]
pub fn Bilibili(
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
        "M7.172 2.757L10.414 6h3.171l3.243-3.242a1 1 0 0 1 1.415 1.415l-1.829 1.827L18.5 6A3.5 3.5 0 0 1 22 9.5v8a3.5 3.5 0 0 1-3.5 3.5h-13A3.5 3.5 0 0 1 2 17.5v-8A3.5 3.5 0 0 1 5.5 6h2.085L5.757 4.171a1 1 0 0 1 1.415-1.415zM18.5 8h-13a1.5 1.5 0 0 0-1.493 1.356L4 9.5v8a1.5 1.5 0 0 0 1.356 1.493L5.5 19h13a1.5 1.5 0 0 0 1.493-1.356L20 17.5v-8A1.5 1.5 0 0 0 18.5 8zM8 11a1 1 0 0 1 1 1v2a1 1 0 0 1-2 0v-2a1 1 0 0 1 1-1zm8 0a1 1 0 0 1 1 1v2a1 1 0 0 1-2 0v-2a1 1 0 0 1 1-1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
