#[cfg(feature = "ImVolumeLow")]
use leptos::*;
#[cfg(feature = "ImVolumeLow")]
///This icon requires the feature `ImVolumeLow` to be enabled.
#[component]
pub fn VolumeLow(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M8.578 11.578c-0.192 0-0.384-0.073-0.53-0.22-0.293-0.293-0.293-0.768 0-1.061 1.267-1.267 1.267-3.329 0-4.596-0.293-0.293-0.293-0.768 0-1.061s0.768-0.293 1.061 0c1.852 1.852 1.852 4.865 0 6.718-0.146 0.146-0.338 0.22-0.53 0.22z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M6.5 15c-0.13 0-0.258-0.051-0.354-0.146l-3.854-3.854h-1.793c-0.276 0-0.5-0.224-0.5-0.5v-5c0-0.276 0.224-0.5 0.5-0.5h1.793l3.854-3.854c0.143-0.143 0.358-0.186 0.545-0.108s0.309 0.26 0.309 0.462v13c0 0.202-0.122 0.385-0.309 0.462-0.062 0.026-0.127 0.038-0.191 0.038z"
        /> < title > { title } < / title > < / svg >
    }
}
