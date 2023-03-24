#[cfg(feature = "SiImagej")]
use leptos::*;
#[cfg(feature = "SiImagej")]
///This icon requires the feature `SiImagej` to be enabled.
#[component]
pub fn Imagej(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M13.1286 17.5084h-8.072a.187.187 0 0 1-.187-.187v-.4442a.187.187 0 0 1 .187-.187h8.0721a.187.187 0 0 1 .187.187v.4442a.1872.1872 0 0 1-.1871.187zm5.6234-12.195c-1.4233.0033-4.2184-.0098-5.6414-.0065a.4035.4035 0 0 0-.4035.4035v3.6061c0 .2229.1807.4035.4035.4035h1.7475v8.19a1.8275 1.8275 0 0 1-.9112 1.5761 1.8277 1.8277 0 0 1-1.8224 0 1.8276 1.8276 0 0 1-.9113-1.5784H6.941c0 2.1705 1.1677 4.193 3.0473 5.2782.9398.5428 1.9936.8141 3.0474.8141s2.1076-.2713 3.0474-.8139c1.8795-1.0837 3.0444-3.1089 3.0473-5.274V5.692a.3785.3785 0 0 0-.3784-.3786zM7.4546 15.2306h3.276a.6403.6403 0 0 0 .6403-.6403V.6403A.6403.6403 0 0 0 10.7306 0h-3.276a.6403.6403 0 0 0-.6403.6403v13.95c0 .3536.2867.6403.6403.6403z"
        /> < title > { title } < / title > < / svg >
    }
}
