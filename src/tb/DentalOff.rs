#[cfg(feature = "TbDentalOff")]
use leptos::*;
#[cfg(feature = "TbDentalOff")]
///This icon requires the feature `TbDentalOff` to be enabled.
#[component]
pub fn DentalOff(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-dental-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19.277 15.281c.463 -1.75 .723 -3.844 .723 -6.281c0 -3.74 -1.908 -5 -4 -5c-1.423 0 -2.92 .911 -4 1.5c-1.074 -.586 -2.583 -1.5 -4 -1.5m-2.843 1.153c-.707 .784 -1.157 2.017 -1.157 3.847c0 4.899 1.056 8.41 2.671 10.537c.573 .756 1.97 .521 2.567 -.236c.398 -.505 .819 -1.439 1.262 -2.801c.292 -.771 .892 -1.504 1.5 -1.5c.602 0 1.21 .737 1.5 1.5c.443 1.362 .864 2.295 1.262 2.8c.597 .759 2 .993 2.567 .237c.305 -.402 .59 -.853 .852 -1.353"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 5.5l3 1.5" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title
        > < / svg >
    }
}
