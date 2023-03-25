#[cfg(feature = "FaSolidBahtSign")]
use leptos::*;
#[cfg(feature = "FaSolidBahtSign")]
///This icon requires the feature `FaSolidBahtSign` to be enabled.
#[component]
pub fn BahtSign(
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
        "M144 0c-17.7 0-32 14.3-32 32V64H37.6C16.8 64 0 80.8 0 101.6V224v41.7V288 406.3c0 23 18.7 41.7 41.7 41.7H112v32c0 17.7 14.3 32 32 32s32-14.3 32-32V448h32c61.9 0 112-50.1 112-112c0-40.1-21.1-75.3-52.7-95.1C280.3 222.6 288 200.2 288 176c0-61.9-50.1-112-112-112V32c0-17.7-14.3-32-32-32zM112 128v96H64V128h48zm64 96V128c26.5 0 48 21.5 48 48s-21.5 48-48 48zm-64 64v96H64V288h48zm64 96V288h32c26.5 0 48 21.5 48 48s-21.5 48-48 48H176z"
        /> < title > { title } < / title > < / svg >
    }
}
