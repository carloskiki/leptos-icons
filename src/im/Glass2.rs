#[cfg(feature = "ImGlass2")]
use leptos::*;
#[cfg(feature = "ImGlass2")]
///This icon requires the feature `ImGlass2` to be enabled.
#[component]
pub fn Glass2(
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
        "M13.893 2.809c0.118-0.151 0.14-0.355 0.057-0.527s-0.258-0.281-0.45-0.281h-11c-0.191 0-0.366 0.109-0.45 0.281s-0.062 0.377 0.057 0.527l4.893 6.228v5.963h-1.5c-0.276 0-0.5 0.224-0.5 0.5s0.224 0.5 0.5 0.5h5c0.276 0 0.5-0.224 0.5-0.5s-0.224-0.5-0.5-0.5h-1.5v-5.963l4.893-6.228zM12.471 3l-1.571 2h-5.8l-1.571-2h8.943z"
        /> < title > { title } < / title > < / svg >
    }
}
