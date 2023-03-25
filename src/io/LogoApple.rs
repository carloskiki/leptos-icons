#[cfg(feature = "IoLogoApple")]
use leptos::*;
#[cfg(feature = "IoLogoApple")]
///This icon requires the feature `IoLogoApple` to be enabled.
#[component]
pub fn LogoApple(
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
        "M349.13,136.86c-40.32,0-57.36,19.24-85.44,19.24C234.9,156.1,212.94,137,178,137c-34.2,0-70.67,20.88-93.83,56.45-32.52,50.16-27,144.63,25.67,225.11,18.84,28.81,44,61.12,77,61.47h.6c28.68,0,37.2-18.78,76.67-19h.6c38.88,0,46.68,18.89,75.24,18.89h.6c33-.35,59.51-36.15,78.35-64.85,13.56-20.64,18.6-31,29-54.35-76.19-28.92-88.43-136.93-13.08-178.34-23-28.8-55.32-45.48-85.79-45.48Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M340.25,32c-24,1.63-52,16.91-68.4,36.86-14.88,18.08-27.12,44.9-22.32,70.91h1.92c25.56,0,51.72-15.39,67-35.11C333.17,85.89,344.33,59.29,340.25,32Z"
        /> < title > { title } < / title > < / svg >
    }
}
