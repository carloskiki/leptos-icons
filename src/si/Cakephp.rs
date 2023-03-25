#[cfg(feature = "SiCakephp")]
use leptos::*;
#[cfg(feature = "SiCakephp")]
///This icon requires the feature `SiCakephp` to be enabled.
#[component]
pub fn Cakephp(
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
        "M0 13.875v3.745c0 2.067 5.37 3.743 12 3.743V17.62c-6.63 0-12-1.68-12-3.743v-.002zm21.384 2.333L12 13.875v3.745l9.384 2.333C23.02 19.313 24 18.503 24 17.62v-3.745c0 .882-.98 1.692-2.616 2.333zM12 10.133v3.742c-6.627 0-12-1.677-12-3.744V6.38c0-2.064 5.37-3.743 12-3.743 6.625 0 12 1.68 12 3.744v3.75c0 .883-.98 1.69-2.616 2.334L12 10.13v.003z"
        /> < title > { title } < / title > < / svg >
    }
}
