#[cfg(feature = "RiDeviceFillFingerprint2")]
use leptos::*;
#[cfg(feature = "RiDeviceFillFingerprint2")]
///This icon requires the feature `RiDeviceFillFingerprint2` to be enabled.
#[component]
pub fn Fingerprint2(
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
        "M12 1a9 9 0 0 1 9 9v4a8.99 8.99 0 0 1-3.81 7.354c.474-1.522.75-3.131.802-4.797L18 16v-2.001h-2V16l-.003.315a15.932 15.932 0 0 1-1.431 6.315 9.045 9.045 0 0 1-3.574.314 12.935 12.935 0 0 0 2.001-6.52L13 16V9h-2v7l-.004.288a10.95 10.95 0 0 1-2.087 6.167 8.98 8.98 0 0 1-2.626-1.504 7.959 7.959 0 0 0 1.71-4.623L8 16v-6l.005-.2a3.978 3.978 0 0 1 .435-1.625l.114-.207-1.445-1.445a5.969 5.969 0 0 0-1.102 3.18L6 10v6l-.004.225a5.968 5.968 0 0 1-1.121 3.273A8.958 8.958 0 0 1 3 14v-4a9 9 0 0 1 9-9zm0 3c-1.196 0-2.31.35-3.246.953l-.23.156 1.444 1.445a3.977 3.977 0 0 1 1.787-.547L12 6l.2.005a4 4 0 0 1 3.795 3.789L16 10v2h2v-2a6 6 0 0 0-6-6z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
