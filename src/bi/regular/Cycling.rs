#[cfg(feature = "BiRegularCycling")]
use leptos::*;
#[cfg(feature = "BiRegularCycling")]
///This icon requires the feature `BiRegularCycling` to be enabled.
#[component]
pub fn Cycling(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M11 15.414V20h2v-4.586c0-.526-.214-1.042-.586-1.414l-2-2L13 9.414l2 2c.372.372.888.586 1.414.586H20v-2h-3.586l-3.707-3.707a.999.999 0 0 0-1.414 0L8 9.586c-.378.378-.586.88-.586 1.414s.208 1.036.586 1.414l3 3z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "16" cy = "5" r = "2" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 14c-2.206 0-4 1.794-4 4s1.794 4 4 4 4-1.794 4-4-1.794-4-4-4zm0 6c-1.103 0-2-.897-2-2s.897-2 2-2 2 .897 2 2-.897 2-2 2zM6 22c2.206 0 4-1.794 4-4s-1.794-4-4-4-4 1.794-4 4 1.794 4 4 4zm0-6c1.103 0 2 .897 2 2s-.897 2-2 2-2-.897-2-2 .897-2 2-2z"
        /> < title > { title } < / title > < / svg >
    }
}
