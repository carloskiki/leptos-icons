#[cfg(feature = "TbVaccineBottleOff")]
use leptos::*;
#[cfg(feature = "TbVaccineBottleOff")]
///This icon requires the feature `TbVaccineBottleOff` to be enabled.
#[component]
pub fn VaccineBottleOff(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-vaccine-bottle-off" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 5v-1a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-4" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8.7 8.705a1.806 1.806 0 0 1 -.2 .045c-.866 .144 -1.5 .893 -1.5 1.77v8.48a2 2 0 0 0 2 2h6a2 2 0 0 0 2 -2v-2m0 -4v-2.48c0 -.877 -.634 -1.626 -1.5 -1.77a1.795 1.795 0 0 1 -1.5 -1.77v-.98"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M7 12h5m4 0h1" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M7 18h10" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 15h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
