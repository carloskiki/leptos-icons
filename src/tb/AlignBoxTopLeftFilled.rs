#[cfg(feature = "TbAlignBoxTopLeftFilled")]
use leptos::*;
#[cfg(feature = "TbAlignBoxTopLeftFilled")]
///This icon requires the feature `TbAlignBoxTopLeftFilled` to be enabled.
#[component]
pub fn AlignBoxTopLeftFilled(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-align-box-top-left-filled" width = "24" height =
        "24" viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill =
        "none" stroke - linecap = "round" stroke - linejoin = "round" width = { size
        .clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke
        = "none" d = "M0 0h24v24H0z" fill = "none" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M18.222 3a2.778 2.778 0 0 1 2.778 2.778v12.444a2.778 2.778 0 0 1 -2.778 2.778h-12.444a2.778 2.778 0 0 1 -2.778 -2.778v-12.444a2.778 2.778 0 0 1 2.778 -2.778zm-8.222 3a1 1 0 0 0 -1 1v6l.007 .117a1 1 0 0 0 1.993 -.117v-6l-.007 -.117a1 1 0 0 0 -.993 -.883zm3 0a1 1 0 0 0 -1 1v4l.007 .117a1 1 0 0 0 1.993 -.117v-4l-.007 -.117a1 1 0 0 0 -.993 -.883zm-6 0a1 1 0 0 0 -1 1v2l.007 .117a1 1 0 0 0 1.993 -.117v-2l-.007 -.117a1 1 0 0 0 -.993 -.883z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
