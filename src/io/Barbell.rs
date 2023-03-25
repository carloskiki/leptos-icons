#[cfg(feature = "IoBarbell")]
use leptos::*;
#[cfg(feature = "IoBarbell")]
///This icon requires the feature `IoBarbell` to be enabled.
#[component]
pub fn Barbell(
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
        "M467,176a29.94,29.94,0,0,0-25.32,12.5,2,2,0,0,1-3.64-1.14V150.71c0-20.75-16.34-38.21-37.08-38.7A38,38,0,0,0,362,150v82a2,2,0,0,1-2,2H152a2,2,0,0,1-2-2V150.71c0-20.75-16.34-38.21-37.08-38.7A38,38,0,0,0,74,150v37.38a2,2,0,0,1-3.64,1.14A29.94,29.94,0,0,0,45,176c-16.3.51-29,14.31-29,30.62v98.72c0,16.31,12.74,30.11,29,30.62a29.94,29.94,0,0,0,25.32-12.5A2,2,0,0,1,74,324.62v36.67C74,382,90.34,399.5,111.08,400A38,38,0,0,0,150,362V280a2,2,0,0,1,2-2H360a2,2,0,0,1,2,2v81.29c0,20.75,16.34,38.21,37.08,38.7A38,38,0,0,0,438,362V324.62a2,2,0,0,1,3.64-1.14A29.94,29.94,0,0,0,467,336c16.3-.51,29-14.31,29-30.62V206.64C496,190.33,483.26,176.53,467,176Z"
        /> < title > { title } < / title > < / svg >
    }
}
