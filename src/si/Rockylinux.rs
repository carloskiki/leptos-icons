#[cfg(feature = "SiRockylinux")]
use leptos::*;
#[cfg(feature = "SiRockylinux")]
///This icon requires the feature `SiRockylinux` to be enabled.
#[component]
pub fn Rockylinux(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M23.332 15.957c.433-1.239.668-2.57.668-3.957 0-6.627-5.373-12-12-12S0 5.373 0 12c0 3.28 1.315 6.251 3.447 8.417L15.62 8.245l3.005 3.005zm-2.192 3.819l-5.52-5.52L6.975 22.9c1.528.706 3.23 1.1 5.025 1.1 3.661 0 6.94-1.64 9.14-4.224z"
        /> < title > { title } < / title > < / svg >
    }
}
