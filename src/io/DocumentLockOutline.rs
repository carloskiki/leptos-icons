#[cfg(feature = "IoDocumentLockOutline")]
use leptos::*;
#[cfg(feature = "IoDocumentLockOutline")]
///This icon requires the feature `IoDocumentLockOutline` to be enabled.
#[component]
pub fn DocumentLockOutline(
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
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        = "M288,304V286c0-16.63-14.26-30-32-30s-32,13.37-32,30v18" fill = "none" stroke =
        "#000" stroke - linecap = "round" stroke - linejoin = "round" stroke - width =
        "32" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M304,416H208a32,32,0,0,1-32-32V336a32,32,0,0,1,32-32h96a32,32,0,0,1,32,32v48A32,32,0,0,1,304,416Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M416,221.25V416a48,48,0,0,1-48,48H144a48,48,0,0,1-48-48V96a48,48,0,0,1,48-48h98.75a32,32,0,0,1,22.62,9.37L406.63,198.63A32,32,0,0,1,416,221.25Z"
        fill = "none" stroke = "#000" stroke - linejoin = "round" stroke - width = "32"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,50.88V176a32,32,0,0,0,32,32H413.12" fill = "none" stroke = "#000" stroke -
        linecap = "round" stroke - linejoin = "round" stroke - width = "32" /> < title >
        { title } < / title > < / svg >
    }
}
