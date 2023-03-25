#[cfg(feature = "FaSolidVolumeOff")]
use leptos::*;
#[cfg(feature = "FaSolidVolumeOff")]
///This icon requires the feature `FaSolidVolumeOff` to be enabled.
#[component]
pub fn VolumeOff(
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
        stroke_witdh = "0" style = style viewBox = "0 0 320 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M320 64c0-12.6-7.4-24-18.9-29.2s-25-3.1-34.4 5.3L131.8 160H64c-35.3 0-64 28.7-64 64v64c0 35.3 28.7 64 64 64h67.8L266.7 471.9c9.4 8.4 22.9 10.4 34.4 5.3S320 460.6 320 448V64z"
        /> < title > { title } < / title > < / svg >
    }
}
