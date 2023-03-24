#[cfg(feature = "RiDeviceLineAirplay")]
use leptos::*;
#[cfg(feature = "RiDeviceLineAirplay")]
///This icon requires the feature `RiDeviceLineAirplay` to be enabled.
#[component]
pub fn Airplay(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M12.4 13.533l5 6.667a.5.5 0 0 1-.4.8H7a.5.5 0 0 1-.4-.8l5-6.667a.5.5 0 0 1 .8 0zM12 16.33L10 19h4l-2-2.67zM18 19v-2h2V5H4v12h2v2H2.992A.994.994 0 0 1 2 18V4c0-.552.455-1 .992-1h18.016c.548 0 .992.445.992 1v14c0 .552-.455 1-.992 1H18z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
