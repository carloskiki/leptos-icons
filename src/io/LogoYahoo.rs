#[cfg(feature = "IoLogoYahoo")]
use leptos::*;
#[cfg(feature = "IoLogoYahoo")]
///This icon requires the feature `IoLogoYahoo` to be enabled.
#[component]
pub fn LogoYahoo(
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
        "M410.32,37.13c-13.56,0-27-.93-39.12-5.13L256,218.67,140.8,32c-12.12,4.2-24.84,5.13-38.4,5.13C89.08,37.13,75.88,36.08,64,32L217.6,280.15V480c12-4.08,25-5.13,38.4-5.13s26.4,1.05,38.4,5.13V280.5L448,32C436.12,36,423.64,37.13,410.32,37.13Z"
        /> < title > { title } < / title > < / svg >
    }
}
