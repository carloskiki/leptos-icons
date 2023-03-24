#[cfg(feature = "SiIconify")]
use leptos::*;
#[cfg(feature = "SiIconify")]
///This icon requires the feature `SiIconify` to be enabled.
#[component]
pub fn Iconify(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M12 19.5c3.75 0 7.159-3.379 6.768-4.125-.393-.75-2.268 1.875-6.768 1.875s-6-2.625-6.375-1.875S8.25 19.5 12 19.5zm4.125-12c.623 0 1.125.502 1.125 1.125v1.5c0 .623-.502 1.125-1.125 1.125A1.123 1.123 0 0115 10.125v-1.5c0-.623.502-1.125 1.125-1.125zm-8.25 0C8.498 7.5 9 8.002 9 8.625v1.5c0 .623-.502 1.125-1.125 1.125a1.123 1.123 0 01-1.125-1.125v-1.5c0-.623.502-1.125 1.125-1.125zM12 0C5.381 0 0 5.381 0 12s5.381 12 12 12 12-5.381 12-12S18.619 0 12 0zm0 1.5c5.808 0 10.5 4.692 10.5 10.5S17.808 22.5 12 22.5 1.5 17.808 1.5 12 6.192 1.5 12 1.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
