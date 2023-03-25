#[cfg(feature = "SiGravatar")]
use leptos::*;
#[cfg(feature = "SiGravatar")]
///This icon requires the feature `SiGravatar` to be enabled.
#[component]
pub fn Gravatar(
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
        "M12 0c-1.326 0-2.4 1.074-2.4 2.4v8.4c0 1.324 1.074 2.398 2.4 2.398s2.4-1.074 2.4-2.398V5.21c2.795.99 4.799 3.654 4.799 6.789 0 3.975-3.225 7.199-7.199 7.199S4.801 15.975 4.801 12c0-1.989.805-3.789 2.108-5.091.938-.938.938-2.458 0-3.396s-2.458-.938-3.396 0C1.344 5.686 0 8.686 0 12c0 6.627 5.373 12 12 12s12-5.373 12-12S18.627 0 12 0"
        /> < title > { title } < / title > < / svg >
    }
}
