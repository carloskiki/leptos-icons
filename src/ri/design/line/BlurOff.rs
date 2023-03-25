#[cfg(feature = "RiDesignLineBlurOff")]
use leptos::*;
#[cfg(feature = "RiDesignLineBlurOff")]
///This icon requires the feature `RiDesignLineBlurOff` to be enabled.
#[component]
pub fn BlurOff(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M18.154 19.568A9 9 0 0 1 5.432 6.846L1.393 2.808l1.415-1.415 19.799 19.8-1.415 1.414-3.038-3.04zM6.847 8.262a7 7 0 0 0 9.891 9.89l-9.89-9.89zM20.414 16.2l-1.599-1.599a6.995 6.995 0 0 0-1.865-6.55L12 3.1 9.657 5.443 8.243 4.03 12 .272l6.364 6.364a9.002 9.002 0 0 1 2.05 9.564z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
