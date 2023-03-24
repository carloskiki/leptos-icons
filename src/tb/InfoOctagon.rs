#[cfg(feature = "TbInfoOctagon")]
use leptos::*;
#[cfg(feature = "TbInfoOctagon")]
///This icon requires the feature `TbInfoOctagon` to be enabled.
#[component]
pub fn InfoOctagon(
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
        "icon icon-tabler icon-tabler-info-octagon" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.103 2h5.794a3 3 0 0 1 2.122 .879l4.101 4.101a3 3 0 0 1 .88 2.123v5.794a3 3 0 0 1 -.879 2.122l-4.101 4.101a3 3 0 0 1 -2.122 .879h-5.795a3 3 0 0 1 -2.122 -.879l-4.101 -4.1a3 3 0 0 1 -.88 -2.123v-5.794a3 3 0 0 1 .879 -2.122l4.101 -4.101a3 3 0 0 1 2.123 -.88z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 8l.01 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 12l1 0l0 4l1 0" /> < title > { title } < /
        title > < / svg >
    }
}
