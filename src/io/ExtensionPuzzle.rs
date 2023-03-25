#[cfg(feature = "IoExtensionPuzzle")]
use leptos::*;
#[cfg(feature = "IoExtensionPuzzle")]
///This icon requires the feature `IoExtensionPuzzle` to be enabled.
#[component]
pub fn ExtensionPuzzle(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M345.14,480H274a18,18,0,0,1-18-18V434.29a31.32,31.32,0,0,0-9.71-22.77c-7.78-7.59-19.08-11.8-30.89-11.51-21.36.5-39.4,19.3-39.4,41.06V462a18,18,0,0,1-18,18H87.62A55.62,55.62,0,0,1,32,424.38V354a18,18,0,0,1,18-18H77.71c9.16,0,18.07-3.92,25.09-11A42.06,42.06,0,0,0,115,295.08C114.7,273.89,97.26,256,76.91,256H50a18,18,0,0,1-18-18V167.62A55.62,55.62,0,0,1,87.62,112h55.24a8,8,0,0,0,8-8V97.52A65.53,65.53,0,0,1,217.54,32c35.49.62,64.36,30.38,64.36,66.33V104a8,8,0,0,0,8,8h55.24A54.86,54.86,0,0,1,400,166.86V222.1a8,8,0,0,0,8,8h5.66c36.58,0,66.34,29,66.34,64.64,0,36.61-29.39,66.4-65.52,66.4H408a8,8,0,0,0-8,8v56A54.86,54.86,0,0,1,345.14,480Z"
        /> < title > { title } < / title > < / svg >
    }
}
