#[cfg(feature = "RiDesignLineShape")]
use leptos::*;
#[cfg(feature = "RiDesignLineShape")]
///This icon requires the feature `RiDesignLineShape` to be enabled.
#[component]
pub fn Shape(
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
        "M7.83 20A3.001 3.001 0 1 1 4 16.17V7.83A3.001 3.001 0 1 1 7.83 4h8.34A3.001 3.001 0 1 1 20 7.83v8.34A3.001 3.001 0 1 1 16.17 20H7.83zm0-2h8.34A3.008 3.008 0 0 1 18 16.17V7.83A3.008 3.008 0 0 1 16.17 6H7.83A3.008 3.008 0 0 1 6 7.83v8.34A3.008 3.008 0 0 1 7.83 18zM5 6a1 1 0 1 0 0-2 1 1 0 0 0 0 2zm14 0a1 1 0 1 0 0-2 1 1 0 0 0 0 2zm0 14a1 1 0 1 0 0-2 1 1 0 0 0 0 2zM5 20a1 1 0 1 0 0-2 1 1 0 0 0 0 2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
