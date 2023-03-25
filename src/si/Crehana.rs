#[cfg(feature = "SiCrehana")]
use leptos::*;
#[cfg(feature = "SiCrehana")]
///This icon requires the feature `SiCrehana` to be enabled.
#[component]
pub fn Crehana(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12,0C5.371,0,0,5.371,0,12c0,6.626,5.371,12,12,12s12-5.374,12-12C24,5.371,18.626,0,12,0z M17.94,9.843v7.915h-3.957 v-3.892h-3.895v3.83H6.13v-3.957h3.833V9.843H6.06V5.948h3.957v3.895h3.965V5.948h3.957V9.843z"
        /> < title > { title } < / title > < / svg >
    }
}
