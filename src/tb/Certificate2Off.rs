#[cfg(feature = "TbCertificate2Off")]
use leptos::*;
#[cfg(feature = "TbCertificate2Off")]
///This icon requires the feature `TbCertificate2Off` to be enabled.
#[component]
pub fn Certificate2Off(
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
        "icon icon-tabler icon-tabler-certificate-2-off" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 12a3 3 0 1 0 3 3" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M11 7h3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 18v4l2 -1l2 1v-4" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M10 19h-2a2 2 0 0 1 -2 -2v-11m1.18 -2.825c.25 -.112 .529 -.175 .82 -.175h8a2 2 0 0 1 2 2v9m-.175 3.82a2 2 0 0 1 -1.825 1.18h-2"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > {
        title } < / title > < / svg >
    }
}
