#[cfg(feature = "SiInteractjs")]
use leptos::*;
#[cfg(feature = "SiInteractjs")]
///This icon requires the feature `SiInteractjs` to be enabled.
#[component]
pub fn Interactjs(
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
        "M12.382.01C12.255.006 12.128 0 12 0A11.999 11.999 0 0 0 1.804 18.327l9.911-17.17zm7.097 19.686L11.201 5.121 2.788 19.689l.007.007h16.684zm.184 1.538H4.337a11.998 11.998 0 0 0 15.326 0zm2.917-3.568A11.999 11.999 0 0 0 12.382.01l.667 1.148zM12.383.009l-.001.001h.001V.009z"
        /> < title > { title } < / title > < / svg >
    }
}
