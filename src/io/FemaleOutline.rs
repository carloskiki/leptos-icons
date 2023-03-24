#[cfg(feature = "IoFemaleOutline")]
use leptos::*;
#[cfg(feature = "IoFemaleOutline")]
///This icon requires the feature `IoFemaleOutline` to be enabled.
#[component]
pub fn FemaleOutline(
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
        "http://www.w3.org/2000/svg" > < circle xmlns = "http://www.w3.org/2000/svg" cx =
        "256" cy = "184" r = "152" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - linejoin = "round" stroke - width = "32" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "256" y1 = "336" x2 = "256" y2 = "480" fill =
        "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin = "round"
        stroke - width = "32" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "314" y1
        = "416" x2 = "198" y2 = "416" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - linejoin = "round" stroke - width = "32" /> < title > { title }
        < / title > < / svg >
    }
}
