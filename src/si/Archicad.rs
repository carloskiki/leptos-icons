#[cfg(feature = "SiArchicad")]
use leptos::*;
#[cfg(feature = "SiArchicad")]
///This icon requires the feature `SiArchicad` to be enabled.
#[component]
pub fn Archicad(
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
        "M22.5896 16.3222c-.779 0-1.4104-.6315-1.4104-1.4105 0-.779.6314-1.4104 1.4104-1.4104S24 14.1328 24 14.9117c0 .779-.6315 1.4105-1.4104 1.4105zM.1507 19.8272c-.35.6959-.0696 1.5438.6263 1.8938.6959.35 1.5438.0695 1.8938-.6263 0 0 7.8494-16.0114 14.2545-16.1487 4.2299-.0907 4.2313 5.642 4.2313 5.642 0 .779.6314 1.4104 1.4104 1.4104s1.4104-.6314 1.4104-1.4104c0 0 .0566-8.3813-7.0196-8.4569C8.7634 1.8711.1507 19.8272.1507 19.8272z"
        /> < title > { title } < / title > < / svg >
    }
}
