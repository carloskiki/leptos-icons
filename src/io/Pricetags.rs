#[cfg(feature = "IoPricetags")]
use leptos::*;
#[cfg(feature = "IoPricetags")]
///This icon requires the feature `IoPricetags` to be enabled.
#[component]
pub fn Pricetags(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M448,183.8v-123A44.66,44.66,0,0,0,403.29,16H280.36a30.62,30.62,0,0,0-21.51,8.89L13.09,270.58a44.86,44.86,0,0,0,0,63.34l117,117a44.84,44.84,0,0,0,63.33,0L439.11,205.31A30.6,30.6,0,0,0,448,183.8ZM352,144a32,32,0,1,1,32-32A32,32,0,0,1,352,144Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M496,64a16,16,0,0,0-16,16V207.37L218.69,468.69a16,16,0,1,0,22.62,22.62l262-262A29.84,29.84,0,0,0,512,208V80A16,16,0,0,0,496,64Z"
        /> < title > { title } < / title > < / svg >
    }
}
