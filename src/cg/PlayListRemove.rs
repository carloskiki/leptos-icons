#[cfg(feature = "CgPlayListRemove")]
use leptos::*;
#[cfg(feature = "CgPlayListRemove")]
///This icon requires the feature `CgPlayListRemove` to be enabled.
#[component]
pub fn PlayListRemove(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.9644 4.63379H3.96442V6.63379H15.9644V4.63379Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.9644 8.63379H3.96442V10.6338H15.9644V8.63379Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.96442 12.6338H11.9644V14.6338H3.96442V12.6338Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.9645 13.7093L14.3787 12.295L16.5 14.4163L18.6213 12.2951L20.0355 13.7093L17.9142 15.8305L20.0356 17.9519L18.6214 19.3661L16.5 17.2447L14.3786 19.3661L12.9644 17.9519L15.0858 15.8305L12.9645 13.7093Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
