#[cfg(feature = "SiCliqz")]
use leptos::*;
#[cfg(feature = "SiCliqz")]
///This icon requires the feature `SiCliqz` to be enabled.
#[component]
pub fn Cliqz(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19.387 18.146l4.19-1.402L12 12.027l4.716 11.578 1.403-4.19 3.917 3.917 1.268-1.268zm-7.387 1c.035 0 .07-.004.105-.004l1.908 4.686c-.654.11-1.326.172-2.013.172-6.617 0-12-5.383-12-12S5.383 0 12 0s12 5.383 12 12c0 .695-.063 1.376-.177 2.04l-4.683-1.908c0-.044.006-.087.006-.133A7.153 7.153 0 0 0 12 4.854a7.155 7.154 0 0 0-7.147 7.145A7.155 7.154 0 0 0 12 19.146z"
        /> < title > { title } < / title > < / svg >
    }
}
