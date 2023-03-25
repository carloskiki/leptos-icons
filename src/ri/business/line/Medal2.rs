#[cfg(feature = "RiBusinessLineMedal2")]
use leptos::*;
#[cfg(feature = "RiBusinessLineMedal2")]
///This icon requires the feature `RiBusinessLineMedal2` to be enabled.
#[component]
pub fn Medal2(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M12 8.5l2.116 5.088 5.492.44-4.184 3.584 1.278 5.36L12 20.1l-4.702 2.872 1.278-5.36-4.184-3.584 5.492-.44L12 8.5zm0 5.207l-.739 1.777-1.916.153 1.46 1.251-.447 1.871L12 17.756l1.641 1.003-.446-1.87 1.459-1.252-1.915-.153L12 13.707zM8 2v9H6V2h2zm10 0v9h-2V2h2zm-5 0v5h-2V2h2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
