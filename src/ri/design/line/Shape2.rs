#[cfg(feature = "RiDesignLineShape2")]
use leptos::*;
#[cfg(feature = "RiDesignLineShape2")]
///This icon requires the feature `RiDesignLineShape2` to be enabled.
#[component]
pub fn Shape2(
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
        "M20 16h2v6h-6v-2H8v2H2v-6h2V8H2V2h6v2h8V2h6v6h-2v8zm-2 0V8h-2V6H8v2H6v8h2v2h8v-2h2zM4 4v2h2V4H4zm0 14v2h2v-2H4zM18 4v2h2V4h-2zm0 14v2h2v-2h-2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
