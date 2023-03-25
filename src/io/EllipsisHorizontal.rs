#[cfg(feature = "IoEllipsisHorizontal")]
use leptos::*;
#[cfg(feature = "IoEllipsisHorizontal")]
///This icon requires the feature `IoEllipsisHorizontal` to be enabled.
#[component]
pub fn EllipsisHorizontal(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < circle xmlns = "http://www.w3.org/2000/svg" cx =
        "256" cy = "256" r = "48" />< circle xmlns = "http://www.w3.org/2000/svg" cx =
        "416" cy = "256" r = "48" />< circle xmlns = "http://www.w3.org/2000/svg" cx =
        "96" cy = "256" r = "48" /> < title > { title } < / title > < / svg >
    }
}
