#[cfg(feature = "SiCountingworkspro")]
use leptos::*;
#[cfg(feature = "SiCountingworkspro")]
///This icon requires the feature `SiCountingworkspro` to be enabled.
#[component]
pub fn Countingworkspro(
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
        "M11.327.512c-3.806.034-7.447 3.19-7.181 7.75.102 1.547.88 3.382 2.981 5.733a.365.365 0 0 0 .635-.23l.053-2.266a.36.36 0 0 0-.1-.255 5.047 5.047 0 0 1 3.407-8.502c2.27-.104 4.011 1.236 4.753 2.744.34.693.527 1.45.55 2.222a.357.357 0 0 0 .343.344c.482.015.962.064 1.437.147a.359.359 0 0 0 .424-.344 7.237 7.237 0 0 0-2.769-5.788C15.02 1.404 13.564.52 11.327.512zm4.94 8.362a14.8 14.8 0 0 0-2.515.26.364.364 0 0 0-.17.635l1.695 1.435a.36.36 0 0 0 .316.073 5.026 5.026 0 0 1 3.123.281c1.78.787 2.92 2.414 3.042 4.304.208 3.187-2.48 5.539-5.277 5.37a5.032 5.032 0 0 1-1.751-.412.365.365 0 0 0-.443.115c-.289.385-.603.75-.94 1.094a.367.367 0 0 0 .09.573c1.887 1.073 3.936 1.16 6.014.32 3.303-1.304 4.63-4.523 4.545-6.847-.096-2.641-1.48-5.072-4.085-6.402-.921-.47-2.04-.812-3.643-.799zm-12.931 1.2a.364.364 0 0 0-.152.052c-1.41.827-2.216 2.057-2.798 3.777-.285.892-.386 1.51-.386 2.436a7.276 7.276 0 0 0 7.157 7.141c1.129.017 2.104-.235 2.962-.583 1.45-.62 3.142-1.597 4.65-4.912a.363.363 0 0 0-.459-.489l-2.365.867a.357.357 0 0 0-.195.174 5.03 5.03 0 0 1-2.268 2.224C6 22.428 2.473 19.784 2.235 16.74c-.145-1.741.494-3.053 1.37-3.982.293-.308.41-.477.663-.662a.36.36 0 0 0 .098-.471 9.173 9.173 0 0 1-.653-1.326.366.366 0 0 0-.377-.225z"
        /> < title > { title } < / title > < / svg >
    }
}
