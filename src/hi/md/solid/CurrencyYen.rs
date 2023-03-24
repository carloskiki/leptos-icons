#[cfg(feature = "HiMdSolidCurrencyYen")]
use leptos::*;
#[cfg(feature = "HiMdSolidCurrencyYen")]
///This icon requires the feature `HiMdSolidCurrencyYen` to be enabled.
#[component]
pub fn CurrencyYen(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM7.34581 5.29439C7.09419 4.96536 6.62349 4.9026 6.29445 5.15421C5.96542 5.40582 5.90266 5.87653 6.15427 6.20557L9.05588 9.99998H6.75004C6.33583 9.99998 6.00004 10.3358 6.00004 10.75C6.00004 11.1642 6.33583 11.5 6.75004 11.5H9.25004V12.5H6.75004C6.33583 12.5 6.00004 12.8358 6.00004 13.25C6.00004 13.6642 6.33583 14 6.75004 14H9.25004V15.25C9.25004 15.6642 9.58583 16 10 16C10.4143 16 10.75 15.6642 10.75 15.25V14H13.25C13.6643 14 14 13.6642 14 13.25C14 12.8358 13.6643 12.5 13.25 12.5H10.75V11.5H13.25C13.6643 11.5 14 11.1642 14 10.75C14 10.3358 13.6643 9.99998 13.25 9.99998H10.9442L13.8458 6.20557C14.0974 5.87653 14.0347 5.40582 13.7056 5.15421C13.3766 4.9026 12.9059 4.96536 12.6543 5.29439L10 8.76531L7.34581 5.29439Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
