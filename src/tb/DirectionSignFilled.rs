#[cfg(feature = "TbDirectionSignFilled")]
use leptos::*;
#[cfg(feature = "TbDirectionSignFilled")]
///This icon requires the feature `TbDirectionSignFilled` to be enabled.
#[component]
pub fn DirectionSignFilled(
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
        "icon icon-tabler icon-tabler-direction-sign-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.52 2.614a2.095 2.095 0 0 1 2.835 -.117l.126 .117l7.905 7.905c.777 .777 .816 2.013 .117 2.836l-.117 .126l-7.905 7.905a2.094 2.094 0 0 1 -2.836 .117l-.126 -.117l-7.907 -7.906a2.096 2.096 0 0 1 -.115 -2.835l.117 -.126l7.905 -7.905zm5.969 9.535l.01 -.116l-.003 -.12l-.016 -.114l-.03 -.11l-.044 -.112l-.052 -.098l-.076 -.105l-.07 -.081l-3.5 -3.5l-.095 -.083a1 1 0 0 0 -1.226 0l-.094 .083l-.083 .094a1 1 0 0 0 0 1.226l.083 .094l1.792 1.793h-5.085l-.117 .007a1 1 0 0 0 0 1.986l.117 .007h5.085l-1.792 1.793l-.083 .094a1 1 0 0 0 1.403 1.403l.094 -.083l3.5 -3.5l.097 -.112l.05 -.074l.037 -.067l.05 -.112l.023 -.076l.025 -.117z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
