#[cfg(feature = "FaSolidBrazilianRealSign")]
use leptos::*;
#[cfg(feature = "FaSolidBrazilianRealSign")]
///This icon requires the feature `FaSolidBrazilianRealSign` to be enabled.
#[component]
pub fn BrazilianRealSign(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M400 0c17.7 0 32 14.3 32 32V50.2c12.5 2.3 24.7 6.4 36.2 12.1l10.1 5.1c15.8 7.9 22.2 27.1 14.3 42.9s-27.1 22.2-42.9 14.3l-10.2-5.1c-9.9-5-20.9-7.5-32-7.5h-1.7c-29.8 0-53.9 24.1-53.9 53.9c0 22 13.4 41.8 33.9 50l52 20.8c44.7 17.9 74.1 61.2 74.1 109.4v3.4c0 51.2-33.6 94.6-80 109.2V480c0 17.7-14.3 32-32 32s-32-14.3-32-32V460.6c-15-3.5-29.4-9.7-42.3-18.3l-23.4-15.6c-14.7-9.8-18.7-29.7-8.9-44.4s29.7-18.7 44.4-8.9L361.2 389c10.8 7.2 23.4 11 36.3 11c27.9 0 50.5-22.6 50.5-50.5v-3.4c0-22-13.4-41.8-33.9-50l-52-20.8C317.3 257.4 288 214.1 288 165.9C288 114 321.5 70 368 54.2V32c0-17.7 14.3-32 32-32zM0 64C0 46.3 14.3 32 32 32h80c79.5 0 144 64.5 144 144c0 58.8-35.2 109.3-85.7 131.7l51.4 128.4c6.6 16.4-1.4 35-17.8 41.6s-35-1.4-41.6-17.8L106.3 320H64V448c0 17.7-14.3 32-32 32s-32-14.3-32-32V288 64zM64 256h48c44.2 0 80-35.8 80-80s-35.8-80-80-80H64V256z"
        /> < title > { title } < / title > < / svg >
    }
}
