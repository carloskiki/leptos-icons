#[cfg(feature = "FaSolidSailboat")]
use leptos::*;
#[cfg(feature = "FaSolidSailboat")]
///This icon requires the feature `FaSolidSailboat` to be enabled.
#[component]
pub fn Sailboat(
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
        "M251 16c0-7 4.5-13.2 11.2-15.3s13.9 .4 17.9 6.1l224 320c3.4 4.9 3.8 11.3 1.1 16.6s-8.2 8.6-14.2 8.6H267c-8.8 0-16-7.2-16-16V16zM207.1 96.5c7 1.9 11.9 8.2 11.9 15.5V336c0 8.8-7.2 16-16 16H75c-5.7 0-11-3-13.8-8s-2.9-11-.1-16l128-224c3.6-6.3 11-9.4 18-7.5zM.7 404.3C-2.2 394.1 5.5 384 16.1 384H549.9c10.6 0 18.3 10.1 15.4 20.3l-4 14.3C545.7 473.9 495.4 512 438 512H128C70.6 512 20.3 473.9 4.7 418.7l-4-14.3z"
        /> < title > { title } < / title > < / svg >
    }
}
