#[cfg(feature = "RiLogosLineQq")]
use leptos::*;
#[cfg(feature = "RiLogosLineQq")]
///This icon requires the feature `RiLogosLineQq` to be enabled.
#[component]
pub fn Qq(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M17.535 12.514l-.696-1.796c0-.021.01-.375.01-.558C16.848 7.088 15.446 4 12 4c-3.446 0-4.848 3.088-4.848 6.16 0 .183.009.537.01.558l-.696 1.796c-.19.515-.38 1.05-.517 1.51-.657 2.189-.444 3.095-.282 3.115.348.043 1.354-1.648 1.354-1.648 0 .98.488 2.258 1.542 3.18-.394.127-.878.32-1.188.557-.28.214-.245.431-.194.52.22.385 3.79.245 4.82.125 1.03.12 4.599.26 4.82-.126.05-.088.085-.305-.194-.519-.311-.237-.795-.43-1.19-.556 1.055-.923 1.542-2.202 1.542-3.181 0 0 1.007 1.691 1.355 1.648.162-.02.378-.928-.283-3.116-.14-.463-.325-.994-.516-1.509zm1.021 8.227c-.373.652-.833.892-1.438 1.057-.24.065-.498.108-.794.138-.44.045-.986.065-1.613.064a33.23 33.23 0 0 1-2.71-.116c-.692.065-1.785.114-2.71.116a16.07 16.07 0 0 1-1.614-.064 4.928 4.928 0 0 1-.793-.138c-.605-.164-1.065-.405-1.44-1.059a2.274 2.274 0 0 1-.239-1.652c-.592-.132-1.001-.483-1.279-.911a2.43 2.43 0 0 1-.309-.71 4.028 4.028 0 0 1-.116-1.106c.013-.785.187-1.762.532-2.912.14-.466.327-1.008.568-1.655l.553-1.43a15.496 15.496 0 0 1-.002-.203C5.152 5.605 7.588 2 12 2c4.413 0 6.848 3.605 6.848 8.16l-.001.203.553 1.43.01.026c.225.606.413 1.153.556 1.626.348 1.15.522 2.129.535 2.916.007.407-.03.776-.118 1.108-.066.246-.161.48-.31.708-.276.427-.684.776-1.277.91.13.554.055 1.14-.24 1.654z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
