#[cfg(feature = "AiTwotoneWallet")]
use leptos::*;
#[cfg(feature = "AiTwotoneWallet")]
///This icon requires the feature `AiTwotoneWallet` to be enabled.
#[component]
pub fn Wallet(
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
        stroke_witdh = "0" style = style viewBox = "0 0 1024 1024" width = { size.clone()
        } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" fill = "#333" d
        =
        "M880 112H144c-17.7 0-32 14.3-32 32v736c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V144c0-17.7-14.3-32-32-32zm-40 464H528V448h312v128zm0-192H496c-17.7 0-32 14.3-32 32v192c0 17.7 14.3 32 32 32h344v200H184V184h656v200z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill = "#E6E6E6" d =
        "M528 576h312V448H528v128zm92-104c22.1 0 40 17.9 40 40s-17.9 40-40 40-40-17.9-40-40 17.9-40 40-40z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill = "#333" d =
        "M580 512a40 40 0 1 0 80 0 40 40 0 1 0-80 0z" />< path xmlns =
        "http://www.w3.org/2000/svg" fill = "#E6E6E6" d =
        "M184 840h656V640H496c-17.7 0-32-14.3-32-32V416c0-17.7 14.3-32 32-32h344V184H184v656z"
        /> < title > { title } < / title > < / svg >
    }
}
