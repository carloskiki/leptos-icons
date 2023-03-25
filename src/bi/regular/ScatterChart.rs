#[cfg(feature = "BiRegularScatterChart")]
use leptos::*;
#[cfg(feature = "BiRegularScatterChart")]
///This icon requires the feature `BiRegularScatterChart` to be enabled.
#[component]
pub fn ScatterChart(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d = "M4 21h17v-2H5V3H3v17a1 1 0 0 0 1 1z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "10" cy = "8" r = "2" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "18" cy = "12" r = "2" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "11.5" cy = "13.5" r = "1.5" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "16.5" cy = "6.5" r = "1.5" /> <
        title > { title } < / title > < / svg >
    }
}
