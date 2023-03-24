#[cfg(feature = "RiLogosFillSpectrum")]
use leptos::*;
#[cfg(feature = "RiLogosFillSpectrum")]
///This icon requires the feature `RiLogosFillSpectrum` to be enabled.
#[component]
pub fn Spectrum(
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
        "M13.2 2.006C21.24 2.093 22 3.25 22 12l-.006 1.2C21.907 21.24 20.75 22 12 22l-1.2-.006c-7.658-.083-8.711-1.136-8.794-8.795L2 11.691l.006-.89c.085-7.85 1.19-8.76 9.382-8.8l1.811.005zM8.25 7h-.583a.667.667 0 0 0-.66.568L7 7.667v3.666c0 .335.247.612.568.66l.099.007h.583a3.75 3.75 0 0 1 3.745 3.55l.005.2v.583c0 .335.247.612.568.66l.099.007h3.666a.667.667 0 0 0 .66-.568l.007-.099v-.583a8.75 8.75 0 0 0-8.492-8.746L8.25 7z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
