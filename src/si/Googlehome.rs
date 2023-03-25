#[cfg(feature = "SiGooglehome")]
use leptos::*;
#[cfg(feature = "SiGooglehome")]
///This icon requires the feature `SiGooglehome` to be enabled.
#[component]
pub fn Googlehome(
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
        "M12 0a1.44 1.44 0 0 0-.947.399L.547 10.762a1.26 1.26 0 0 0-.342.808v11.138c0 .768.53 1.292 1.311 1.292h20.968c.78 0 1.311-.522 1.311-1.292V11.57a1.25 1.25 0 0 0-.34-.804L15.68 3.097h-.001L12.947.4A1.454 1.454 0 0 0 12 0Zm0 6.727 6.552 6.456v5.65H5.446v-5.65z"
        /> < title > { title } < / title > < / svg >
    }
}
