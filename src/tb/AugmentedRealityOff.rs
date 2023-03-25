#[cfg(feature = "TbAugmentedRealityOff")]
use leptos::*;
#[cfg(feature = "TbAugmentedRealityOff")]
///This icon requires the feature `TbAugmentedRealityOff` to be enabled.
#[component]
pub fn AugmentedRealityOff(
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
        "icon icon-tabler icon-tabler-augmented-reality-off" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 8v-2c0 -.557 .228 -1.061 .595 -1.424" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 16v2a2 2 0 0 0 2 2h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 4h2a2 2 0 0 1 2 2v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 20h2c.558 0 1.062 -.228 1.425 -.596" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12.5l.312 -.195m2.457 -1.536l1.231 -.769" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M9.225 9.235l-1.225 .765l4 2.5v4.5l3.076 -1.923m.924 -3.077v-2l-4 -2.5l-.302 .189"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M8 10v4.5l4 2.5" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < /
        title > < / svg >
    }
}
