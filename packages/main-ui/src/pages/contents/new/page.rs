#![allow(non_snake_case)]

use crate::components::headings::Heading1;

use super::controller::*;
use super::i18n::*;
use by_components::files::DropZone;
#[allow(unused_imports)]
use by_components::icons;
use dioxus::prelude::*;
use dioxus_translate::*;
use dto::ContentCreateRequest;

#[component]
pub fn NewContentsPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: NewContentsTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: "{tr.title}" }

        div {
            id: "new-contents",
            class: "w-full flex flex-col items-start gap-[10px] pb-[50px]",
            div { class: "flex flex-col items-start gap-[20px]",
                Heading1 { lang, "{tr.title}" }
                pre {
                    class: "text-black text-[14px] font-normal",
                    font_family: "Pretendard",
                    "{tr.description}"
                }
            }
            div { class: "w-full flex flex-row justify-end",
                button {
                    class: "min-w-[125px] px-[20px] h-[44px] bg-white text-black font-bold text-[16px] rounded-[12px] hover:bg-[#24B28C] hover:text-white transition-all duration-500 ease-in-out",
                    box_shadow: "0px 4px 20px rgba(84, 157, 159, 0.25)",
                    onclick: move |_| ctrl.add_content(),
                    "{tr.btn_add_nft}"
                }
            }
            for i in 0..ctrl.contents().len() {
                SingleContent {
                    lang,
                    onchange: move |req| ctrl.set_content(i, req),
                    ondelete: move || ctrl.handle_delete(i),
                }
            }

            div { class: "fixed bottom-0 left-0 w-full h-[110px] bg-white z-[10] flex flex-row items-center justify-center",
                div { class: "w-full max-w-[1440px] flex flex-row justify-between items-center max-[1440px]:px-[20px]",
                    div {
                    }

                    div { class: "flex flex-row items-center gap-[20px] text-white font-bold text-[16px]",
                        button {
                            class: "bg-gray-500 h-[50px] py-[15px] px-[24px] hover:bg-gray-700 rounded-[12px]",
                            onclick: move |_| ctrl.handle_cancel(),
                            "{tr.btn_cancel}"
                        }
                        button {
                            class: "bg-[#24B28C] h-[50px] py-[15px] px-[24px] hover:bg-[#34a39d] rounded-[12px]",
                            onclick: move |_| async move {
                                let _ = ctrl.handle_submit().await;
                            },
                            "{tr.btn_submit_nft}"
                        }
                    }
                }


            }
        }
    }
}

#[component]
pub fn SingleContent(
    lang: Language,
    onchange: EventHandler<ContentCreateRequest>,
    ondelete: EventHandler<()>,
) -> Element {
    let mut dropping = use_signal(|| false);
    let bg = if dropping() {
        "bg-[#FF2D55]/5 border-[#FF2D55]"
    } else {
        "border-[#dfdfdf]"
    };
    let tr: NewContentsTranslate = translate(&lang);
    let mut title = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());
    let mut thumbnail = use_signal(|| None);
    let mut source: Signal<Option<(String, String)>> = use_signal(|| None);

    let send = move || {
        let req = ContentCreateRequest {
            title: title(),
            description: description(),
            thumbnail_image: thumbnail().unwrap_or_default(),
            source: if source().is_some() {
                source().unwrap().0
            } else {
                thumbnail().unwrap_or_default()
            },
            ..Default::default()
        };
        onchange.call(req);
    };

    rsx! {
        div { class: "w-full flex flex-col items-start justify-start gap-[30px] bg-white rounded-[12px] py-[30px] px-[20px]",
            InputWithLabel {
                label: "{tr.label_title}",
                placeholder: "{tr.placeholder_title}",
                max: 30,
                oninput: move |e| {
                    if title().chars().count() < 30 {
                        title.set(e);
                        send();
                    }
                },
                value: title(),
                multiline: false,
                mandatory: true,
            }

            div { class: "w-full flex flex-col gap-[10px] items-start justify-start",
                label { class: "text-[#5B5B5B] font-bold text-[14px] flex flex-row items-center",
                    span { "{tr.label_thumbnail}" }
                    span { class: "text-[#FF0000]", "*" }
                }

                div { class: "w-full p-[16px] flex flex-col items-start justify-start rounded-[12px] border-[1px] border-[#dfdfdf] text-[#979797] font-normal text-[15px] bg-transparent gap-[16px]",

                    p { class: "font-bold text-[14px] text-[#8d8d8d]", "{tr.label_fileupload}" }

                    DropZone {
                        class: "w-full border-[1px] rounded-[14px] flex flex-col items-center justify-center  border-dashed gap-[16px] {bg}",
                        onupload: move |(file_bytes, ext)| async move {
                            let (uri, _) = handle_upload(file_bytes, ext).await?;
                            thumbnail.set(Some(uri));
                            send();
                            Ok(())
                        },
                        onchange: move |hover| dropping.set(hover),
                        if let Some(image) = thumbnail() {
                            img {
                                class: "w-full object-cover rounded-[12px]",
                                src: image,
                            }
                        } else {
                            div { class: "h-[145px] flex flex-col items-center justify-center gap-[16px]",
                                Document {}
                                div { class: "flex flex-col gap-[8px] items-center justify-center",
                                    p { class: "text-[14px] text-[#8d8d8d]",
                                        "{tr.placeholder_fileupload}"
                                    }
                                    p { class: "text-[12px] text-[#8d8d8d]", "{tr.note_fileupload}" }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "w-full flex flex-col gap-[10px] items-start justify-start",
                label { class: "text-[#5B5B5B] font-bold text-[14px] flex flex-row items-center",
                    span { "{tr.label_source}" }
                    span { class: "text-[#FF0000]", "*" }
                }

                div { class: "w-full p-[16px] flex flex-col items-start justify-start rounded-[12px] border-[1px] border-[#dfdfdf] text-[#979797] font-normal text-[15px] bg-transparent gap-[16px]",

                    if let Some(ref s) = source() {
                        div { class: "flex flex-row gap-[10px] items-center justify-between w-full px-[20px] border-[1px] py-[10px] rounded-[12px] border-[#dfdfdf] text-[#979797] font-normal text-[15px] bg-transparent",
                            div { class: "flex flex-col gap-[10px]",
                                p { class: "text-[14px] font-bold", "{s.0}" }
                                p { class: "text-[12px] ", "{s.1}" }
                            }
                            div {
                                class: "cursor-pointer",
                                onclick: move |_| source.set(None),
                                icons::symbols::CloseCircle {}
                            }
                        }
                    } else {
                        DropZone {
                            class: "w-full border-[1px] rounded-[14px] flex flex-col items-center justify-center  border-dashed gap-[16px] {bg}",
                            onupload: move |(file_bytes, ext)| async move {
                                match handle_upload(file_bytes, ext).await {
                                    Ok(uri) => {
                                        source.set(Some(uri));
                                        send();
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to upload source file: {:?}", e);
                                    }
                                }
                            },
                            onchange: move |hover| dropping.set(hover),
                            div { class: "h-[145px] flex flex-col items-center justify-center gap-[16px]",
                                Document {}
                                p { class: "text-[14px] text-[#8d8d8d]", "{tr.placeholder_source}" }
                            }
                        }
                    }
                }
            }

            InputWithLabel {
                label: "{tr.label_description}",
                placeholder: "{tr.placeholder_description}",
                max: 300,
                oninput: move |e| {
                    if description().chars().count() < 300 {
                        description.set(e);
                        send();
                    }
                },
                value: description(),
                multiline: true,
                mandatory: false,
            }

            div { class: "w-full flex flex-row items-center justify-end",
                button {
                    class: "rounded-[12px] border-[1px] border-[#CACACA] flex flex-row items-center justify-center h-[44px] py-[14px] px-[24px] text-[14px] font-bold text-[#191919] hover:bg-[#F5F5F5] transition-all duration-500 ease-in-out",
                    onclick: move |_| ondelete(()),
                    by_components::icons::edit::Delete3 {}
                    span { "{tr.btn_delete}" }
                }
            }
        } // end of this page
    }
}

#[component]
pub fn InputWithLabel(
    label: String,
    placeholder: String,
    max: usize,
    value: String,
    oninput: EventHandler<String>,
    multiline: bool,
    mandatory: bool,
) -> Element {
    rsx! {
        div { class: "relative w-full flex flex-col gap-[10px] items-start justify-start",

            label { class: "text-[#5B5B5B] font-bold text-[14px] flex flex-row items-center",
                span { "{label}" }
                if mandatory {
                    span { class: "text-[#FF0000]", "*" }
                }
            }

            if multiline {
                textarea {
                    class: "w-full px-[24px] py-[10px] flex flex-row items-center justify-start rounded-[12px] border-[1px] border-[#dfdfdf] text-[#979797] font-normal text-[15px] bg-transparent",
                    placeholder: "{placeholder}",
                    value: "{value}",
                    maxlength: max,
                    oninput: move |evt| oninput(evt.value()),
                    rows: "5",
                }
            } else {
                input {
                    class: "w-full px-[24px] h-[45px] flex flex-row items-center justify-start rounded-[12px] border-[1px] border-[#dfdfdf] text-[#979797] font-normal text-[15px] bg-transparent",
                    r#type: if multiline { "textarea" },
                    maxlength: max,
                    value: "{value}",
                    placeholder: "{placeholder}",
                    oninput: move |e| oninput(e.value()),

                }
            }
            p {
                class: "absolute right-[10px] bottom-[0px] h-[45px] flex flex-row items-center justify-center text-[#979797] font-normal text-[14px] z-[10]",
                color: if value.chars().count() == max { "#FF0000" },
                "{value.chars().count()}/{max}"
            }

        }
    }
}

#[component]
pub fn Document(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "49",
            view_box: "0 0 49 49",
            width: "49",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M36.9842 18.7329L25.3922 7.18884C24.5042 6.30084 23.3042 5.79688 22.0562 5.79688H21.6002H12.0002C9.3602 5.79688 7.2002 7.95688 7.2002 10.5969V39.3969C7.2002 42.0369 9.3602 44.1969 12.0002 44.1969H33.6002C36.2402 44.1969 38.4002 42.0369 38.4002 39.3969V22.1409C38.4002 20.8689 37.8962 19.6449 36.9842 18.7329ZM24.0002 9.20491L35.0402 20.1969H26.4002C25.0802 20.1969 24.0002 19.1169 24.0002 17.7969V9.20491ZM36.0002 39.3969C36.0002 40.7169 34.9202 41.7969 33.6002 41.7969H12.0002C10.6802 41.7969 9.6002 40.7169 9.6002 39.3969V10.5969C9.6002 9.27688 10.6802 8.19688 12.0002 8.19688H21.6002V17.7969C21.6002 20.4369 23.7602 22.5969 26.4002 22.5969H36.0002V39.3969Z",
                fill: "#B1B1B1",
            }
            path {
                d: "M13.2002 28.5984C13.2002 29.2704 13.7282 29.7984 14.4002 29.7984H31.2002C31.8722 29.7984 32.4002 29.2704 32.4002 28.5984C32.4002 27.9264 31.8722 27.3984 31.2002 27.3984H14.4002C13.7282 27.3984 13.2002 27.9264 13.2002 28.5984Z",
                fill: "#B1B1B1",
            }
            path {
                d: "M14.4002 22.5953H18.0002C18.6722 22.5953 19.2002 22.0673 19.2002 21.3953C19.2002 20.7233 18.6722 20.1953 18.0002 20.1953H14.4002C13.7282 20.1953 13.2002 20.7233 13.2002 21.3953C13.2002 22.0673 13.7282 22.5953 14.4002 22.5953Z",
                fill: "#B1B1B1",
            }
            path {
                d: "M31.2002 34.5938H14.4002C13.7282 34.5938 13.2002 35.1217 13.2002 35.7938C13.2002 36.4658 13.7282 36.9938 14.4002 36.9938H31.2002C31.8722 36.9938 32.4002 36.4658 32.4002 35.7938C32.4002 35.1217 31.8722 34.5938 31.2002 34.5938Z",
                fill: "#B1B1B1",
            }
            path {
                d: "M47.0672 5.26667H43.3338V1.53333C43.3338 1.23467 43.0992 1 42.8005 1C42.5018 1 42.2672 1.23467 42.2672 1.53333V5.26667H38.5338C38.2352 5.26667 38.0005 5.50133 38.0005 5.8C38.0005 6.09867 38.2352 6.33333 38.5338 6.33333H42.2672V10.0667C42.2672 10.3653 42.5018 10.6 42.8005 10.6C43.0992 10.6 43.3338 10.3653 43.3338 10.0667V6.33333H47.0672C47.3658 6.33333 47.6005 6.09867 47.6005 5.8C47.6005 5.50133 47.3658 5.26667 47.0672 5.26667Z",
                fill: "#B1B1B1",
                stroke: "#B1B1B1",
            }
        }
    }
}
