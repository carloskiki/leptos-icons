#[cfg(feature = "IoLogoUsd")]
use leptos::*;
#[cfg(feature = "IoLogoUsd")]
///This icon requires the feature `IoLogoUsd` to be enabled.
#[component]
pub fn LogoUsd(
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
        "M240,480V443.58C160.53,439,112.25,398.06,112,336h72c1.77,26.34,23.86,46.45,56,50V288L213.23,281c-61-14.18-93.64-49.39-93.64-102.08C119.59,116.81,164.08,76.08,240,70V32h32V70c77.39,6.3,119,47.74,120,106H320c-.76-24.06-15.83-43.39-48-46v92l30.82,7.28C367.61,243.46,400,277,400,332c0,64.34-43.74,105.88-128,111.32V480Zm0-264V130c-27.59,1.52-47.27,18.47-47.27,42.53C192.73,194.83,209.12,209.41,240,216Zm32,78v92c38.15-1.54,56.38-18.92,56.38-45.77C328.38,315.65,310.15,299.1,272,294Z"
        /> < title > { title } < / title > < / svg >
    }
}
