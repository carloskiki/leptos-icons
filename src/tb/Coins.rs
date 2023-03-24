#[cfg(feature = "TbCoins")]
use leptos::*;
#[cfg(feature = "TbCoins")]
///This icon requires the feature `TbCoins` to be enabled.
#[component]
pub fn Coins(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-coins"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 14c0 1.657 2.686 3 6 3s6 -1.343 6 -3s-2.686 -3 -6 -3s-6 1.343 -6 3z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M9 14v4c0 1.656 2.686 3 6 3s6 -1.344 6 -3v-4" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3 6c0 1.072 1.144 2.062 3 2.598s4.144 .536 6 0c1.856 -.536 3 -1.526 3 -2.598c0 -1.072 -1.144 -2.062 -3 -2.598s-4.144 -.536 -6 0c-1.856 .536 -3 1.526 -3 2.598z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 6v10c0 .888 .772 1.45 2 2"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 11c0 .888 .772 1.45 2 2" />
        < title > { title } < / title > < / svg >
    }
}
