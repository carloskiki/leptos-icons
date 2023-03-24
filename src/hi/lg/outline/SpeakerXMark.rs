#[cfg(feature = "HiLgOutlineSpeakerXMark")]
use leptos::*;
#[cfg(feature = "HiLgOutlineSpeakerXMark")]
///This icon requires the feature `HiLgOutlineSpeakerXMark` to be enabled.
#[component]
pub fn SpeakerXMark(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.25 9.75041L19.5 12.0004M19.5 12.0004L21.75 14.2504M19.5 12.0004L21.75 9.75041M19.5 12.0004L17.25 14.2504M6.75 8.25041L11.4697 3.53074C11.9421 3.05827 12.75 3.3929 12.75 4.06107V19.9398C12.75 20.6079 11.9421 20.9426 11.4697 20.4701L6.75 15.7504H4.50905C3.62971 15.7504 2.8059 15.2439 2.57237 14.3962C2.36224 13.6334 2.25 12.83 2.25 12.0004C2.25 11.1708 2.36224 10.3675 2.57237 9.60465C2.8059 8.75689 3.62971 8.25041 4.50905 8.25041H6.75Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
