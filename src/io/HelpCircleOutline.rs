#[cfg(feature = "IoHelpCircleOutline")]
use leptos::*;
#[cfg(feature = "IoHelpCircleOutline")]
///This icon requires the feature `IoHelpCircleOutline` to be enabled.
#[component]
pub fn HelpCircleOutline(
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
        "M256,80A176,176,0,1,0,432,256,176,176,0,0,0,256,80Z" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M200,202.29s.84-17.5,19.57-32.57C230.68,160.77,244,158.18,256,158c10.93-.14,20.69,1.67,26.53,4.45,10,4.76,29.47,16.38,29.47,41.09,0,26-17,37.81-36.37,50.8S251,281.43,251,296"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:28px"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "250" cy = "348" r = "20" />
        < title > { title } < / title > < / svg >
    }
}
