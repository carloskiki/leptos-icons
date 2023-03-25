#[cfg(feature = "RiDesignFillShape2")]
use leptos::*;
#[cfg(feature = "RiDesignFillShape2")]
///This icon requires the feature `RiDesignFillShape2` to be enabled.
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
        "M2 2h5v5H2V2zm0 15h5v5H2v-5zM17 2h5v5h-5V2zm0 15h5v5h-5v-5zM8 4h8v2H8V4zM4 8h2v8H4V8zm14 0h2v8h-2V8zM8 18h8v2H8v-2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
