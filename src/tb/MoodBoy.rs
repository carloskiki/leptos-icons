#[cfg(feature = "TbMoodBoy")]
use leptos::*;
#[cfg(feature = "TbMoodBoy")]
///This icon requires the feature `TbMoodBoy` to be enabled.
#[component]
pub fn MoodBoy(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-mood-boy"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 4.5a9 9 0 0 1 3.864 5.89a2.5 2.5 0 0 1 -.29 4.36a9 9 0 0 1 -17.137 0a2.5 2.5 0 0 1 -.29 -4.36a9 9 0 0 1 3.746 -5.81"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M9.5 16a3.5 3.5 0 0 0 5 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M8.5 2c1.5 1 2.5 3.5 2.5 5" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M12.5 2c1.5 2 2 3.5 2 5" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M9 12l.01 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 12l.01 0" /> < title > { title } < / title
        > < / svg >
    }
}
