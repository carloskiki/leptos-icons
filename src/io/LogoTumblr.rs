#[cfg(feature = "IoLogoTumblr")]
use leptos::*;
#[cfg(feature = "IoLogoTumblr")]
///This icon requires the feature `IoLogoTumblr` to be enabled.
#[component]
pub fn LogoTumblr(
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
        "M390,32H120c-49.19,0-88,38.81-88,88V390c0,49.19,38.81,90,88,90H390c49.19,0,90-40.81,90-90V120C480,70.81,439.19,32,390,32ZM336,396H284c-42.51,0-72-23.68-72-76V240H176V192c42.51-11,57.95-48.32,60-80h44v72h52v56H280l-.39,70.51c0,21.87,11,29.43,28.62,29.43L336,340Z"
        /> < title > { title } < / title > < / svg >
    }
}
