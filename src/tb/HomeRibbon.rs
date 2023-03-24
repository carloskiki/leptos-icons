#[cfg(feature = "TbHomeRibbon")]
use leptos::*;
#[cfg(feature = "TbHomeRibbon")]
///This icon requires the feature `TbHomeRibbon` to be enabled.
#[component]
pub fn HomeRibbon(
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
        "icon icon-tabler icon-tabler-home-ribbon" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16 15h5v7l-2.5 -1.5l-2.5 1.5z" />< path xmlns = "http://www.w3.org/2000/svg" d
        = "M20 11l-8 -8l-9 9h2v7a2 2 0 0 0 2 2h5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 21v-6a2 2 0 0 1 2 -2h1.5" /> < title > {
        title } < / title > < / svg >
    }
}
