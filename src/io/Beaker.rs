#[cfg(feature = "IoBeaker")]
use leptos::*;
#[cfg(feature = "IoBeaker")]
///This icon requires the feature `IoBeaker` to be enabled.
#[component]
pub fn Beaker(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M444,32H128c-19.38,0-45.9,4.34-64.11,24.77C52.17,69.92,48,85.66,48,96a16,16,0,0,0,13.8,15.85C91.7,116,96,117.79,96,136V400A80.07,80.07,0,0,0,176,480H368a80.11,80.11,0,0,0,80-80V96c0-12.55,7.46-27.25,10-31.36l.1-.14c.22-.35.5-.72.78-1.1,2-2.79,5.09-7,5.09-12.95C464,39.79,454.89,32,444,32ZM84.11,83.08c5.24-8.87,17.17-19,44.29-19H422.83C419.3,72.87,416,84.27,416,96v64H128V136C128,98.68,106.65,87.86,84.11,83.08Z"
        /> < title > { title } < / title > < / svg >
    }
}
