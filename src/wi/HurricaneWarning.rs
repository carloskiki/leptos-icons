#[cfg(feature = "WiHurricaneWarning")]
use leptos::*;
#[cfg(feature = "WiHurricaneWarning")]
///This icon requires the feature `WiHurricaneWarning` to be enabled.
#[component]
pub fn HurricaneWarning(
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
        stroke_witdh = "0" style = style version = "1.1" id = "Layer_1" x = "0px" y =
        "0px" viewBox = "0 0 30 30" style = "enable-background:new 0 0 30 30;" space =
        "preserve" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" d =
        "M7.8,24.6V7.45h1.13V24.6H7.8z M9.73,21.52v-6.6h8.55v6.6H9.73z M9.73,14.05v-6.6h8.55v6.6H9.73z M12.09,19.52h3.81v-2.51&#xA;	h-3.81V19.52z M12.09,12.05h3.81v-2.5h-3.81V12.05z"
        /> < title > { title } < / title > < / svg >
    }
}
