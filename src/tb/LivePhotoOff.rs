#[cfg(feature = "TbLivePhotoOff")]
use leptos::*;
#[cfg(feature = "TbLivePhotoOff")]
///This icon requires the feature `TbLivePhotoOff` to be enabled.
#[component]
pub fn LivePhotoOff(
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
        "icon icon-tabler icon-tabler-live-photo-off" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.296 11.29a1 1 0 1 0 1.414 1.415" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8.473 8.456a5 5 0 1 0 7.076 7.066m1.365 -2.591a5 5 0 0 0 -5.807 -5.851" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M15.9 20.11v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.04 17.61v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20.77 14v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20.77 10v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.04 6.39v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15.9 3.89v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 3v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8.1 3.89v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4.96 6.39v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3.23 10v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3.23 14v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4.96 17.61v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8.1 20.11v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 21v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
