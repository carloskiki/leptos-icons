#[cfg(feature = "IoHelpSharp")]
use leptos::*;
#[cfg(feature = "IoHelpSharp")]
///This icon requires the feature `IoHelpSharp` to be enabled.
#[component]
pub fn HelpSharp(
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
        "M160,164c0-10,1.44-33,33.54-59.46C212.6,88.83,235.49,84.28,256,84c18.73-.23,35.47,2.94,45.48,7.82C318.59,100.2,352,120.6,352,164c0,45.67-29.18,66.37-62.35,89.18S248,290.36,248,316"
        style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:40px"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "220" y = "368" width = "56"
        height = "56" rx = "3.5" ry = "3.5" /> < title > { title } < / title > < / svg >
    }
}
