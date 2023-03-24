#[cfg(feature = "TbDiamondFilled")]
use leptos::*;
#[cfg(feature = "TbDiamondFilled")]
///This icon requires the feature `TbDiamondFilled` to be enabled.
#[component]
pub fn DiamondFilled(
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
        "icon icon-tabler icon-tabler-diamond-filled" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 4a1 1 0 0 1 .783 .378l.074 .108l3 5a1 1 0 0 1 -.032 1.078l-.08 .103l-8.53 9.533a1.7 1.7 0 0 1 -1.215 .51c-.4 0 -.785 -.14 -1.11 -.417l-.135 -.126l-8.5 -9.5a1 1 0 0 1 -.172 -1.067l.06 -.115l3.013 -5.022l.064 -.09a.982 .982 0 0 1 .155 -.154l.089 -.064l.088 -.05l.05 -.023l.06 -.025l.109 -.032l.112 -.02l.117 -.005h12zm-8.886 3.943a1 1 0 0 0 -1.371 .343l-.6 1l-.06 .116a1 1 0 0 0 .177 1.07l2 2.2l.09 .088a1 1 0 0 0 1.323 -.02l.087 -.09a1 1 0 0 0 -.02 -1.323l-1.501 -1.65l.218 -.363l.055 -.103a1 1 0 0 0 -.398 -1.268z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
