#[cfg(feature = "WiNa")]
use leptos::*;
#[cfg(feature = "WiNa")]
///This icon requires the feature `WiNa` to be enabled.
#[component]
pub fn Na(
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
        stroke_witdh = "0" style = style version = "1.1" id = "Layer_1" x = "0px" y =
        "0px" viewBox = "0 0 30 30" style = "enable-background:new 0 0 30 30;" space =
        "preserve" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" d =
        "M6.87,18.23h1.94v-3.64h0.02l2.05,3.64h1.99v-6.66h-1.94v3.55h-0.02l-1.94-3.55h-2.1V18.23z M13.39,18.38h1.43l2.61-6.97&#xA;	h-1.42L13.39,18.38z M16.26,18.23h2.07l0.29-0.95h2.12l0.28,0.95h2.13l-2.43-6.66h-2.01L16.26,18.23z M19.07,15.84l0.64-2.04h0.03&#xA;	l0.6,2.04H19.07z"
        /> < title > { title } < / title > < / svg >
    }
}
