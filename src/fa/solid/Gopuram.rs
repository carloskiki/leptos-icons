#[cfg(feature = "FaSolidGopuram")]
use leptos::*;
#[cfg(feature = "FaSolidGopuram")]
///This icon requires the feature `FaSolidGopuram` to be enabled.
#[component]
pub fn Gopuram(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M120 0c13.3 0 24 10.7 24 24v8h40V24c0-13.3 10.7-24 24-24s24 10.7 24 24v8h48V24c0-13.3 10.7-24 24-24s24 10.7 24 24v8h40V24c0-13.3 10.7-24 24-24s24 10.7 24 24v8V64v64c17.7 0 32 14.3 32 32v64c17.7 0 32 14.3 32 32v96c17.7 0 32 14.3 32 32v96c0 17.7-14.3 32-32 32H416V352H384V224H352V128H320v96h32V352h32V512H304V464c0-26.5-21.5-48-48-48s-48 21.5-48 48v48H128V352h32V224h32V128H160v96H128V352H96V512H32c-17.7 0-32-14.3-32-32V384c0-17.7 14.3-32 32-32V256c0-17.7 14.3-32 32-32V160c0-17.7 14.3-32 32-32V64 32 24c0-13.3 10.7-24 24-24zM256 272c-17.7 0-32 14.3-32 32v48h64V304c0-17.7-14.3-32-32-32zm-32-80v32h64V192c0-17.7-14.3-32-32-32s-32 14.3-32 32z"
        /> < title > { title } < / title > < / svg >
    }
}
