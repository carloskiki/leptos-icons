#[cfg(feature = "TiGroup")]
use leptos::*;
#[cfg(feature = "TiGroup")]
///This icon requires the feature `TiGroup` to be enabled.
#[component]
pub fn Group(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 14c1.381 0 2.631-.56 3.536-1.465.904-.904 1.464-2.154 1.464-3.535s-.56-2.631-1.464-3.535c-.905-.905-2.155-1.465-3.536-1.465s-2.631.56-3.536 1.465c-.904.904-1.464 2.154-1.464 3.535s.56 2.631 1.464 3.535c.905.905 2.155 1.465 3.536 1.465zM20 15c.69 0 1.315-.279 1.768-.731.453-.452.732-1.077.732-1.769 0-.69-.279-1.315-.732-1.768-.453-.453-1.078-.732-1.768-.732-.691 0-1.316.279-1.769.732-.452.453-.731 1.078-.731 1.768 0 .691.279 1.316.731 1.769s1.078.731 1.769.731zM20 15.59c-1.331 0-2.332.406-2.917.968-1.115-.917-2.878-1.558-5.083-1.558-2.266 0-3.995.648-5.092 1.564-.596-.565-1.608-.974-2.908-.974-2.188 0-3.5 1.09-3.5 2.182 0 .545 1.312 1.092 3.5 1.092.604 0 1.146-.051 1.623-.133l-.04.27c0 1 2.406 2 6.417 2 3.762 0 6.417-1 6.417-2l-.02-.255c.463.073.995.118 1.603.118 2.051 0 3.5-.547 3.5-1.092 0-1.092-1.373-2.182-3.5-2.182zM4 15c.69 0 1.315-.279 1.768-.732.453-.453.732-1.078.732-1.768 0-.689-.279-1.314-.732-1.768-.453-.452-1.078-.732-1.768-.732-.691 0-1.316.28-1.769.732-.452.454-.731 1.079-.731 1.768 0 .69.279 1.315.731 1.768.453.453 1.078.732 1.769.732z"
        /> < title > { title } < / title > < / svg >
    }
}
