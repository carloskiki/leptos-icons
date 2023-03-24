#[cfg(feature = "VsLayersDot")]
use leptos::*;
#[cfg(feature = "VsLayersDot")]
///This icon requires the feature `VsLayersDot` to be enabled.
#[component]
pub fn LayersDot(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M8.18535 1.08325L7.62706 1.08717L1.71796 5.12422L1.72152 5.95233L7.63062 9.91528L8.1818 9.91912L14.2727 5.95617L14.2762 5.1203L8.18535 1.08325ZM2.89198 5.53323L7.91335 2.10268L13.0891 5.5332L7.91329 8.90079L2.89198 5.53323ZM7.63059 12.4153L1.79257 8.5H3.58794L7.91326 11.4008L12.3716 8.5H14.2053L13.4056 9.02031C13.2722 9.00688 13.1369 9 13 9C11.224 9 9.71839 10.1574 9.19622 11.7591L8.18177 12.4191L7.63059 12.4153ZM9.00447 13.1908L7.91326 13.9008L3.58794 11H1.79257L7.63059 14.9153L8.18177 14.9191L9.20113 14.2559C9.08965 13.9185 9.02187 13.5612 9.00447 13.1908Z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "13" cy = "13" r = "3" /> <
        title > { title } < / title > < / svg >
    }
}
