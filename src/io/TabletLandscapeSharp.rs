#[cfg(feature = "IoTabletLandscapeSharp")]
use leptos::*;
#[cfg(feature = "IoTabletLandscapeSharp")]
///This icon requires the feature `IoTabletLandscapeSharp` to be enabled.
#[component]
pub fn TabletLandscapeSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M0,82V430a18,18,0,0,0,18,18H494a18,18,0,0,0,18-18V82a18,18,0,0,0-18-18H18A18,18,0,0,0,0,82ZM448,412H64V100H448Z"
        /> < title > { title } < / title > < / svg >
    }
}
