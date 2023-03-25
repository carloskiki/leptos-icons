#[cfg(feature = "BiSolidShieldAlt2")]
use leptos::*;
#[cfg(feature = "BiSolidShieldAlt2")]
///This icon requires the feature `BiSolidShieldAlt2` to be enabled.
#[component]
pub fn ShieldAlt2(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M21.881 5.223a.496.496 0 0 0-.747-.412c-.672.392-1.718.898-2.643.898-.421 0-.849-.064-1.289-.198a5.712 5.712 0 0 1-.808-.309c-1.338-.639-2.567-1.767-3.696-2.889a1.008 1.008 0 0 0-.698-.29 1.008 1.008 0 0 0-.698.29c-1.129 1.122-2.358 2.25-3.696 2.889h-.001a5.655 5.655 0 0 1-.807.309c-.44.134-.869.198-1.289.198-.925 0-1.971-.507-2.643-.898a.496.496 0 0 0-.747.412c-.061 1.538-.077 4.84.688 7.444 1.399 4.763 4.48 7.976 8.91 9.292l.14.041.14-.014V22v-.014H12l.143.014.14-.041c4.43-1.316 7.511-4.529 8.91-9.292.765-2.604.748-5.906.688-7.444z"
        /> < title > { title } < / title > < / svg >
    }
}
