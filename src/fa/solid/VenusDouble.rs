#[cfg(feature = "FaSolidVenusDouble")]
use leptos::*;
#[cfg(feature = "FaSolidVenusDouble")]
///This icon requires the feature `FaSolidVenusDouble` to be enabled.
#[component]
pub fn VenusDouble(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M176 288a112 112 0 1 0 0-224 112 112 0 1 0 0 224zM352 176c0 86.3-62.1 158.1-144 173.1V384h32c17.7 0 32 14.3 32 32s-14.3 32-32 32H208v32c0 17.7-14.3 32-32 32s-32-14.3-32-32V448H112c-17.7 0-32-14.3-32-32s14.3-32 32-32h32V349.1C62.1 334.1 0 262.3 0 176C0 78.8 78.8 0 176 0s176 78.8 176 176zM328 318c14.6-15.6 26.8-33.4 36-53c18.8 14.4 42.4 23 68 23c61.9 0 112-50.1 112-112s-50.1-112-112-112c-25.6 0-49.1 8.6-68 23c-9.3-19.5-21.5-37.4-36-53C357.1 12.6 393.1 0 432 0c97.2 0 176 78.8 176 176c0 86.3-62.1 158.1-144 173.1V384h32c17.7 0 32 14.3 32 32s-14.3 32-32 32H464v32c0 17.7-14.3 32-32 32s-32-14.3-32-32V448H368c-17.7 0-32-14.3-32-32s14.3-32 32-32h32V349.1c-26.6-4.9-51.1-15.7-72-31.1z"
        /> < title > { title } < / title > < / svg >
    }
}
