#[cfg(feature = "IoLogoFigma")]
use leptos::*;
#[cfg(feature = "IoLogoFigma")]
///This icon requires the feature `IoLogoFigma` to be enabled.
#[component]
pub fn LogoFigma(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M336,176a80,80,0,0,0,0-160H176a80,80,0,0,0,0,160,80,80,0,0,0,0,160,80,80,0,1,0,80,80V176Z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "336" cy = "256" r = "80" />
        < title > { title } < / title > < / svg >
    }
}
