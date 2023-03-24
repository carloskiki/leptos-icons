#[cfg(feature = "SiTypo3")]
use leptos::*;
#[cfg(feature = "SiTypo3")]
///This icon requires the feature `SiTypo3` to be enabled.
#[component]
pub fn Typo3(
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
        "M18.08 16.539c-.356.105-.64.144-1.012.144-3.048 0-7.524-10.652-7.524-14.197 0-1.305.31-1.74.745-2.114C6.56.808 2.082 2.177.651 3.917c-.31.436-.497 1.12-.497 1.99C.154 11.442 6.06 24 10.228 24c1.928 0 5.178-3.168 7.852-7.46M16.134 0c3.855 0 7.713.622 7.713 2.798 0 4.415-2.8 9.765-4.23 9.765-2.549 0-5.72-7.09-5.72-10.635C13.897.31 14.518 0 16.134 0"
        /> < title > { title } < / title > < / svg >
    }
}
