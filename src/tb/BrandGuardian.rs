#[cfg(feature = "TbBrandGuardian")]
use leptos::*;
#[cfg(feature = "TbBrandGuardian")]
///This icon requires the feature `TbBrandGuardian` to be enabled.
#[component]
pub fn BrandGuardian(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-brand-guardian" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M14 13h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M4 12c0 -9.296 9.5 -9 9.5 -9c-2.808 0 -4.5 4.373 -4.5 9s1.763 8.976 4.572 8.976c0 .023 -9.572 1.092 -9.572 -8.976z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.5 3c1.416 0 3.853 1.16 4.5 2v3.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 13v8s2.77 -.37 4 -2v-6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13.5 21h1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13.5 3h1" /> < title > { title } < / title > <
        / svg >
    }
}
