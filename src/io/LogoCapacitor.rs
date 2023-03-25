#[cfg(feature = "IoLogoCapacitor")]
use leptos::*;
#[cfg(feature = "IoLogoCapacitor")]
///This icon requires the feature `IoLogoCapacitor` to be enabled.
#[component]
pub fn LogoCapacitor(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M480,101.09,372.37,208.72,479.23,315.78l-69.3,69.3L126.71,101.85,196,32.54,303.07,139.42,410.67,32Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M32.55,196l69.3-69.31L385.07,409.93l-69.3,69.3-107-106.87L101.08,480,32,410.67,139.42,303.06Z"
        /> < title > { title } < / title > < / svg >
    }
}
