#[cfg(feature = "SiRome")]
use leptos::*;
#[cfg(feature = "SiRome")]
///This icon requires the feature `SiRome` to be enabled.
#[component]
pub fn Rome(
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
        "M12 0a16.941 16.941 0 00-2.283.154.63.63 0 00-.485.895l.172.361 1.338 2.8a.633.633 0 00.608.356 11.695 11.695 0 011.3 0 .632.632 0 00.608-.355l1.338-2.8.172-.362a.63.63 0 00-.485-.895A16.941 16.941 0 0012 0zm4.309 1.902a.629.629 0 00-.786.32l-.949 1.989a.629.629 0 00.36.863c3.193 1.134 5.466 4.063 5.466 7.498v.627H24v-.627c0-4.864-3.191-9.021-7.691-10.67zm-8.62.002C3.191 3.551 0 7.706 0 12.572v.627h3.6v-.627c0-3.435 2.275-6.364 5.466-7.498a.629.629 0 00.358-.863l-.95-1.986a.629.629 0 00-.785-.32zM12 5.714c-3.976 0-7.2 3.07-7.2 6.858V24h3.6V12.572c0-1.895 1.612-3.43 3.6-3.43s3.6 1.536 3.6 3.43V24h3.6V12.572c0-3.787-3.224-6.857-7.2-6.857zM0 14.4V24h3.6v-9.6zm20.4 0V24H24v-9.6z"
        /> < title > { title } < / title > < / svg >
    }
}
