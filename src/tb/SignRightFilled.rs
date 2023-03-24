#[cfg(feature = "TbSignRightFilled")]
use leptos::*;
#[cfg(feature = "TbSignRightFilled")]
///This icon requires the feature `TbSignRightFilled` to be enabled.
#[component]
pub fn SignRightFilled(
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
        "icon icon-tabler icon-tabler-sign-right-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 2a1 1 0 0 1 .993 .883l.007 .117v2h5a1 1 0 0 1 .694 .28l.087 .095l2 2.5a1 1 0 0 1 .072 1.147l-.072 .103l-2 2.5a1 1 0 0 1 -.652 .367l-.129 .008h-5v8h1a1 1 0 0 1 .117 1.993l-.117 .007h-4a1 1 0 0 1 -.117 -1.993l.117 -.007h1v-8h-3a1 1 0 0 1 -.993 -.883l-.007 -.117v-5a1 1 0 0 1 .883 -.993l.117 -.007h3v-2a1 1 0 0 1 1 -1z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
