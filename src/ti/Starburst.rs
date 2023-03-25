#[cfg(feature = "TiStarburst")]
use leptos::*;
#[cfg(feature = "TiStarburst")]
///This icon requires the feature `TiStarburst` to be enabled.
#[component]
pub fn Starburst(
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
        "M19.064 10.109l1.179-2.387c.074-.149.068-.327-.015-.471-.083-.145-.234-.238-.401-.249l-2.656-.172-.172-2.656c-.011-.167-.104-.317-.249-.401-.145-.084-.322-.09-.472-.015l-2.385 1.18-1.477-2.215c-.186-.278-.646-.278-.832 0l-1.477 2.215-2.385-1.18c-.151-.075-.327-.069-.472.015-.145.083-.238.234-.249.401l-.171 2.656-2.657.171c-.167.011-.318.104-.401.249-.084.145-.089.322-.015.472l1.179 2.386-2.214 1.477c-.139.093-.223.249-.223.416s.083.323.223.416l2.215 1.477-1.18 2.386c-.074.15-.068.327.015.472.083.144.234.238.401.248l2.656.171.171 2.657c.011.167.104.317.249.401.144.083.32.088.472.015l2.386-1.179 1.477 2.214c.093.139.249.223.416.223s.323-.083.416-.223l1.477-2.214 2.386 1.179c.15.073.327.068.472-.015s.238-.234.249-.401l.171-2.656 2.656-.172c.167-.011.317-.104.401-.249.083-.145.089-.322.015-.472l-1.179-2.385 2.214-1.478c.139-.093.223-.249.223-.416s-.083-.323-.223-.416l-2.214-1.475z"
        /> < title > { title } < / title > < / svg >
    }
}
