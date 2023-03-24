#[cfg(feature = "RiMediaLineVolumeOffVibrate")]
use leptos::*;
#[cfg(feature = "RiMediaLineVolumeOffVibrate")]
///This icon requires the feature `RiMediaLineVolumeOffVibrate` to be enabled.
#[component]
pub fn VolumeOffVibrate(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M19.39 3.161l1.413 1.414-2.475 2.475 2.475 2.475L18.328 12l2.475 2.476-2.475 2.475 2.475 2.475-1.414 1.414-3.889-3.89 2.475-2.474L15.5 12l2.475-2.475L15.5 7.05l3.89-3.889zM13 19.945a.5.5 0 0 1-.817.387L6.89 15.999 3 16a1 1 0 0 1-1-1V9a1 1 0 0 1 1-1l2.584-.002-3.776-3.776 1.414-1.414L13 12.586v7.359zM7.584 9.998L4 10V14l3.603-.001L11 16.779v-3.365L7.584 9.998zm5.303-6.26a.5.5 0 0 1 .113.317v5.702l-2-2V7.22l-.296.241-1.421-1.42 2.9-2.373a.5.5 0 0 1 .704.07z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
