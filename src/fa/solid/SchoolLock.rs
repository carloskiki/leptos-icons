#[cfg(feature = "FaSolidSchoolLock")]
use leptos::*;
#[cfg(feature = "FaSolidSchoolLock")]
///This icon requires the feature `FaSolidSchoolLock` to be enabled.
#[component]
pub fn SchoolLock(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M302.2 5.4c10.7-7.2 24.8-7.2 35.5 0L473.7 96H592c26.5 0 48 21.5 48 48V272c0-61.9-50.1-112-112-112s-112 50.1-112 112v24.6c-19.1 11.1-32 31.7-32 55.4H320.3l-.3 0c-35.3 0-64 28.7-64 64v96h64v0H48c-26.5 0-48-21.5-48-48V144c0-26.5 21.5-48 48-48H166.3L302.2 5.4zM80 208v64c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16V208c0-8.8-7.2-16-16-16H96c-8.8 0-16 7.2-16 16zm0 128v64c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16V336c0-8.8-7.2-16-16-16H96c-8.8 0-16 7.2-16 16zm240-72a88 88 0 1 0 0-176 88 88 0 1 0 0 176zm16-120v16h16c8.8 0 16 7.2 16 16s-7.2 16-16 16H320c-8.8 0-16-7.2-16-16V144c0-8.8 7.2-16 16-16s16 7.2 16 16zm192 96c-17.7 0-32 14.3-32 32v48h64V272c0-17.7-14.3-32-32-32zm-80 32c0-44.2 35.8-80 80-80s80 35.8 80 80v48c17.7 0 32 14.3 32 32V480c0 17.7-14.3 32-32 32H448c-17.7 0-32-14.3-32-32V352c0-17.7 14.3-32 32-32V272z"
        /> < title > { title } < / title > < / svg >
    }
}
