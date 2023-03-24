#[cfg(feature = "SiHtmlacademy")]
use leptos::*;
#[cfg(feature = "SiHtmlacademy")]
///This icon requires the feature `SiHtmlacademy` to be enabled.
#[component]
pub fn Htmlacademy(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 0L2.524.994v17.368L12 24l9.476-5.638V.994L12.099.01 12 0zm8.236 17.657L12 22.557l-8.236-4.9v-7.119l8.2 4.881.014.885-5.626-3.349-.008.86 5.648 3.394.015.908-5.647-3.36-.008.86L12 19.01l5.703-3.412v-.862l-.008.004v-2.805l2.54-1.517v7.238zm-.006-8.162l-2.254 1.328-1.04.613-4.96-2.951-.009.858 4.24 2.521-.037.023-.092.054-.602.355-3.5-2.083-.009.859 2.763 1.643-.652.436-.015.01-2.088-1.23-.008.858 1.37.807-1.395.837-8.16-4.85 8.172-4.912v.001l8.276 4.823zm.006-.864l-8.28-4.882h-.002l-8.19 4.877V2.11L12 1.246l8.237.864v6.52z"
        /> < title > { title } < / title > < / svg >
    }
}
