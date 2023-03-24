#[cfg(feature = "VsLayers")]
use leptos::*;
#[cfg(feature = "VsLayers")]
///This icon requires the feature `VsLayers` to be enabled.
#[component]
pub fn Layers(
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
        "M7.62706 1.08717L8.18535 1.08325L14.2762 5.1203L14.2727 5.95617L8.1818 9.91912L7.63062 9.91528L1.72152 5.95233L1.71796 5.12422L7.62706 1.08717ZM7.91335 2.10268L2.89198 5.53323L7.91329 8.90079L13.0891 5.5332L7.91335 2.10268ZM1.79257 8.5L7.63059 12.4153L8.18177 12.4191L14.2053 8.5H12.3716L7.91326 11.4008L3.58794 8.5H1.79257ZM7.63059 14.9153L1.79257 11H3.58794L7.91326 13.9008L12.3716 11H14.2053L8.18177 14.9191L7.63059 14.9153Z"
        /> < title > { title } < / title > < / svg >
    }
}
