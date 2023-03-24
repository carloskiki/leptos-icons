#[cfg(feature = "IoFlashlightOutline")]
use leptos::*;
#[cfg(feature = "IoFlashlightOutline")]
///This icon requires the feature `IoFlashlightOutline` to be enabled.
#[component]
pub fn FlashlightOutline(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M456.64,162.86,349.12,55.36c-13.15-13.14-28.68-7.17-41.82,6l-11.95,12c-26.13,26.13-27.62,58.38-29.42,83.31-.89,12.24-9.78,27.55-18.51,36.28L58.58,381.67C42.23,398,45.89,421.09,62.31,437.51l12.17,12.17c16.36,16.35,39.43,20.16,55.86,3.74l188.83-188.8c8.74-8.74,24-17.55,36.29-18.52,24.87-1.86,58.62-4.85,83.26-29.49l11.94-11.94C463.81,191.53,469.78,176,456.64,162.86Z"
        style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< circle
        xmlns = "http://www.w3.org/2000/svg" cx = "224.68" cy = "287.28" r = "20" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "289" y1 = "81" x2 = "431" y2 =
        "223" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
