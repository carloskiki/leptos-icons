#[cfg(feature = "TbGenderEpicene")]
use leptos::*;
#[cfg(feature = "TbGenderEpicene")]
///This icon requires the feature `TbGenderEpicene` to be enabled.
#[component]
pub fn GenderEpicene(
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
        "icon icon-tabler icon-tabler-gender-epicene" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.536 15.536a5 5 0 1 0 -7.072 -7.072a5 5 0 0 0 7.072 7.072z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15.536 15.535l5.464 -5.535" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 14l5.464 -5.535" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 12h.01" /> < title > { title } < / title >
        < / svg >
    }
}
