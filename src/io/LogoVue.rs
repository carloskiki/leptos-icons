#[cfg(feature = "IoLogoVue")]
use leptos::*;
#[cfg(feature = "IoLogoVue")]
///This icon requires the feature `IoLogoVue` to be enabled.
#[component]
pub fn LogoVue(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < polygon xmlns = "http://www.w3.org/2000/svg"
        points =
        "256 144.03 200.51 47.92 121.08 47.92 256 281.61 390.92 47.92 311.49 47.92 256 144.03"
        />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "409.4 47.92 256 313.61 102.6 47.92 15.74 47.92 256 464.08 496.26 47.92 409.4 47.92"
        /> < title > { title } < / title > < / svg >
    }
}
