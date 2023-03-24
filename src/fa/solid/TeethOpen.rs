#[cfg(feature = "FaSolidTeethOpen")]
use leptos::*;
#[cfg(feature = "FaSolidTeethOpen")]
///This icon requires the feature `FaSolidTeethOpen` to be enabled.
#[component]
pub fn TeethOpen(
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
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M96 32C43 32 0 75 0 128v64c0 35.3 28.7 64 64 64H512c35.3 0 64-28.7 64-64V128c0-53-43-96-96-96H96zM224 96c26.5 0 48 21.5 48 48v56c0 13.3-10.7 24-24 24H200c-13.3 0-24-10.7-24-24V144c0-26.5 21.5-48 48-48zm80 48c0-26.5 21.5-48 48-48s48 21.5 48 48v56c0 13.3-10.7 24-24 24H328c-13.3 0-24-10.7-24-24V144zM96 128c26.5 0 48 21.5 48 48v24c0 13.3-10.7 24-24 24H72c-13.3 0-24-10.7-24-24V176c0-26.5 21.5-48 48-48zm336 48c0-26.5 21.5-48 48-48s48 21.5 48 48v24c0 13.3-10.7 24-24 24H456c-13.3 0-24-10.7-24-24V176zM96 480H480c53 0 96-43 96-96V352c0-35.3-28.7-64-64-64H64c-35.3 0-64 28.7-64 64v32c0 53 43 96 96 96zm0-64c-26.5 0-48-21.5-48-48V344c0-13.3 10.7-24 24-24h48c13.3 0 24 10.7 24 24v24c0 26.5-21.5 48-48 48zm80-48V344c0-13.3 10.7-24 24-24h48c13.3 0 24 10.7 24 24v24c0 26.5-21.5 48-48 48s-48-21.5-48-48zm176 48c-26.5 0-48-21.5-48-48V344c0-13.3 10.7-24 24-24h48c13.3 0 24 10.7 24 24v24c0 26.5-21.5 48-48 48zm80-48V344c0-13.3 10.7-24 24-24h48c13.3 0 24 10.7 24 24v24c0 26.5-21.5 48-48 48s-48-21.5-48-48z"
        /> < title > { title } < / title > < / svg >
    }
}
