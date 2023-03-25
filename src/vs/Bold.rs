#[cfg(feature = "VsBold")]
use leptos::*;
#[cfg(feature = "VsBold")]
///This icon requires the feature `VsBold` to be enabled.
#[component]
pub fn Bold(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 13V3h3.362c1.116 0 1.954.224 2.515.673.565.449.848 1.113.848 1.992 0 .467-.137.881-.41 1.243-.273.357-.645.634-1.116.831.556.151.993.44 1.314.865.325.422.487.925.487 1.511 0 .898-.299 1.603-.897 2.116-.598.513-1.443.769-2.536.769H5zm1.356-4.677v3.599h2.24c.63 0 1.127-.158 1.49-.474.367-.32.55-.76.55-1.319 0-1.204-.673-1.806-2.02-1.806h-2.26zm0-1.058h2.049c.593 0 1.066-.144 1.42-.433.357-.288.536-.68.536-1.174 0-.55-.165-.948-.494-1.195-.33-.252-.831-.378-1.505-.378H6.356v3.18z"
        /> < title > { title } < / title > < / svg >
    }
}
