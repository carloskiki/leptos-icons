#[cfg(feature = "TiVideo")]
use leptos::*;
#[cfg(feature = "TiVideo")]
///This icon requires the feature `TiVideo` to be enabled.
#[component]
pub fn Video(
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
        "M22.525 7.149c-.16-.099-.342-.149-.525-.149-.153 0-.306.035-.447.105l-2.553 1.277v-.382c0-1.654-1.346-3-3-3h-11c-1.654 0-3 1.346-3 3v8c0 1.654 1.346 3 3 3h11c1.654 0 3-1.346 3-3v-.382l2.553 1.276c.141.071.294.106.447.106.183 0 .365-.05.525-.149.295-.183.475-.504.475-.851v-8c0-.347-.18-.668-.475-.851zm-15.525 6.351c-.829 0-1.5-.671-1.5-1.5s.671-1.5 1.5-1.5 1.5.671 1.5 1.5-.671 1.5-1.5 1.5z"
        /> < title > { title } < / title > < / svg >
    }
}
