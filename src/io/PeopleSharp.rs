#[cfg(feature = "IoPeopleSharp")]
use leptos::*;
#[cfg(feature = "IoPeopleSharp")]
///This icon requires the feature `IoPeopleSharp` to be enabled.
#[component]
pub fn PeopleSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < circle xmlns = "http://www.w3.org/2000/svg" cx =
        "152" cy = "184" r = "72" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M234,296c-28.16-14.3-59.24-20-82-20-44.58,0-136,27.34-136,82v42H166V383.93c0-19,8-38.05,22-53.93C199.17,317.32,214.81,305.55,234,296Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M340,288c-52.07,0-156,32.16-156,96v48H496V384C496,320.16,392.07,288,340,288Z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "340" cy = "168" r = "88" />
        < title > { title } < / title > < / svg >
    }
}
