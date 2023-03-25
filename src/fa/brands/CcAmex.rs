#[cfg(feature = "FaBrandsCcAmex")]
use leptos::*;
#[cfg(feature = "FaBrandsCcAmex")]
///This icon requires the feature `FaBrandsCcAmex` to be enabled.
#[component]
pub fn CcAmex(
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
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M48 480C21.49 480 0 458.5 0 432V80C0 53.49 21.49 32 48 32H528C554.5 32 576 53.49 576 80V82.43H500.5L483.5 130L466.6 82.43H369.4V145.6L341.3 82.43H262.7L181 267.1H246.8V430.9H450.5L482.4 395.8L514.3 430.9H576V432C576 458.5 554.5 480 528 480H48zM482.6 364L440.4 410.3H390.5L458 338.6L390.5 266.1H441.9L483.4 312.8L525.4 266.1H576L508 338.2L576 410.3H524.6L482.6 364zM576 296.9V380.2L536.7 338.3L576 296.9zM307.6 377.1H390.6V410.3H268.6V267.1H390.6V300.2H307.6V322.6H388.5V354.9H307.6V377.2V377.1zM537.3 145.7L500.4 246.3H466L429.2 146V246.3H390.5V103H451.7L483.6 192.3L515.8 103H576V246.3H537.3V145.7zM334.5 217.6H268.6L256.7 246.3H213.7L276.1 103H327.3L390.6 246.3H346.5L334.5 217.6zM301.5 138.5L282 185.4H320.9L301.5 138.5z"
        /> < title > { title } < / title > < / svg >
    }
}
