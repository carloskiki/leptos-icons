#[cfg(feature = "IoCloudDone")]
use leptos::*;
#[cfg(feature = "IoCloudDone")]
///This icon requires the feature `IoCloudDone` to be enabled.
#[component]
pub fn CloudDone(
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
        "M424.44,227.25a16,16,0,0,1-12.12-12.39c-7.68-36.68-24.45-68.15-49.18-92A153.57,153.57,0,0,0,256,80c-35.5,0-68.24,11.69-94.68,33.8a156.24,156.24,0,0,0-42,56,16,16,0,0,1-11.37,9.15c-27,5.62-51.07,17.34-69.18,33.87C13.39,235.88,0,267.42,0,304c0,36,14.38,68.88,40.49,92.59C65.64,419.43,99.56,432,136,432H396c32.37,0,60.23-8.57,80.59-24.77C499.76,388.78,512,361.39,512,328,512,270.43,470,237.42,424.44,227.25Zm-95.2-8.94-107.8,128a16,16,0,0,1-12,5.69h-.27a16,16,0,0,1-11.88-5.28l-45.9-50.87c-5.77-6.39-5.82-16.33.3-22.4a16,16,0,0,1,23.16.63l33.9,37.58,96-114a16,16,0,1,1,24.48,20.62Z"
        /> < title > { title } < / title > < / svg >
    }
}
