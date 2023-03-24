#[cfg(feature = "IoLogoMarkdown")]
use leptos::*;
#[cfg(feature = "IoLogoMarkdown")]
///This icon requires the feature `IoLogoMarkdown` to be enabled.
#[component]
pub fn LogoMarkdown(
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
        "M475,64H37C16.58,64,0,81.38,0,102.77V409.19C0,430.59,16.58,448,37,448H475c20.38,0,37-17.41,37-38.81V102.77C512,81.38,495.42,64,475,64ZM288,368H224V256l-48,64-48-64V368H64V144h64l48,80,48-80h64Zm96,0L304,256h48.05L352,144h64V256h48Z"
        /> < title > { title } < / title > < / svg >
    }
}
