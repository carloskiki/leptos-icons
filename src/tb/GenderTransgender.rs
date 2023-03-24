#[cfg(feature = "TbGenderTransgender")]
use leptos::*;
#[cfg(feature = "TbGenderTransgender")]
///This icon requires the feature `TbGenderTransgender` to be enabled.
#[component]
pub fn GenderTransgender(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-gender-transgender" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 9l6 -6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 7v-4h-4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 9l-6 -6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 7v-4h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5.5 8.5l3 -3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 16v5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9.5 19h5" /> < title > { title } < / title > <
        / svg >
    }
}
