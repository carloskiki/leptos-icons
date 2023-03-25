#[cfg(feature = "TiPencil")]
use leptos::*;
#[cfg(feature = "TiPencil")]
///This icon requires the feature `TiPencil` to be enabled.
#[component]
pub fn Pencil(
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
        "M21 6.879l-3.879-3.879c-.293-.293-.678-.439-1.061-.439-.384 0-.767.146-1.06.439l-10.939 10.939c-.293.293-.558.727-.75 1.188-.192.463-.311.959-.311 1.373v4.5h4.5c.414 0 .908-.119 1.371-.311.463-.192.896-.457 1.189-.75l10.94-10.939c.293-.293.439-.678.439-1.061 0-.384-.146-.767-.439-1.06zm-15.232 8.182l8.293-8.293 1.232 1.232-8.293 8.293-1.232-1.232zm1.732 3.939h-1.5l-1-1v-1.5c0-.077.033-.305.158-.605.01-.02 2.967 2.938 2.967 2.938-.322.134-.548.167-.625.167zm1.439-.768l-1.232-1.232 8.293-8.293 1.232 1.232-8.293 8.293zm9-9l-3.172-3.172 1.293-1.293 3.17 3.172-1.291 1.293z"
        /> < title > { title } < / title > < / svg >
    }
}
