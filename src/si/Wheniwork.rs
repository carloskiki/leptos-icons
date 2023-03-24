#[cfg(feature = "SiWheniwork")]
use leptos::*;
#[cfg(feature = "SiWheniwork")]
///This icon requires the feature `SiWheniwork` to be enabled.
#[component]
pub fn Wheniwork(
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
        "M12 24C5.342 24 0 18.582 0 12 0 5.342 5.342 0 12 0s12 5.342 12 12c0 6.582-5.342 12-12 12zm0-21.986c-5.497 0-9.987 4.489-9.987 9.986 0 5.498 4.49 9.988 9.987 9.988 5.498 0 9.987-4.49 9.987-9.988 0-5.497-4.489-9.986-9.987-9.986zm5.885 11.148H9.213c-.384 0-.695-.309-.698-.691v-1.012c0-.387.311-.697.698-.697h8.748c.387 0 .697.311.697.697v1.006c-.077.387-.387.697-.773.697zm-2.246-3.871H6.891c-.383.002-.697-.307-.698-.691V7.59c0-.311.31-.621.697-.621h8.748c.31 0 .62.311.62.619v1.006c.001.386-.31.697-.619.697zm-8.748 5.418h8.748c.388 0 .696.311.696.697v1.006c.002.383-.309.695-.691.697H6.891c-.388-.076-.697-.387-.697-.773V15.33c-.001-.31.309-.621.697-.621z"
        /> < title > { title } < / title > < / svg >
    }
}
