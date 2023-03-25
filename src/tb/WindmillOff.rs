#[cfg(feature = "TbWindmillOff")]
use leptos::*;
#[cfg(feature = "TbWindmillOff")]
///This icon requires the feature `TbWindmillOff` to be enabled.
#[component]
pub fn WindmillOff(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-windmill-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.061 11.06c1.18 -.824 1.939 -2.11 1.939 -3.56c0 -2.49 -2.24 -4.5 -5 -4.5v5"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12c0 2.76 2.01 5 4.5 5c.166 0 .33 -.01 .49 -.03m2.624 -1.36c.856 -.91 1.386 -2.19 1.386 -3.61h-5"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12c-2.76 0 -5 2.01 -5 4.5s2.24 4.5 5 4.5v-9z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M6.981 7.033c-2.244 .285 -3.981 2.402 -3.981 4.967h9" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
