#[cfg(feature = "FaSolidBellSlash")]
use leptos::*;
#[cfg(feature = "FaSolidBellSlash")]
///This icon requires the feature `FaSolidBellSlash` to be enabled.
#[component]
pub fn BellSlash(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M38.8 5.1C28.4-3.1 13.3-1.2 5.1 9.2S-1.2 34.7 9.2 42.9l592 464c10.4 8.2 25.5 6.3 33.7-4.1s6.3-25.5-4.1-33.7l-87.5-68.6c.5-1.7 .7-3.5 .7-5.4c0-27.6-11-54.1-30.5-73.7L512 320c-20.5-20.5-32-48.3-32-77.3V208c0-77.4-55-142-128-156.8V32c0-17.7-14.3-32-32-32s-32 14.3-32 32V51.2c-42.6 8.6-79 34.2-102 69.3L38.8 5.1zM160 242.7c0 29-11.5 56.8-32 77.3l-1.5 1.5C107 341 96 367.5 96 395.2c0 11.5 9.3 20.8 20.8 20.8H406.2L160 222.1v20.7zM384 448H320 256c0 17 6.7 33.3 18.7 45.3s28.3 18.7 45.3 18.7s33.3-6.7 45.3-18.7s18.7-28.3 18.7-45.3z"
        /> < title > { title } < / title > < / svg >
    }
}
