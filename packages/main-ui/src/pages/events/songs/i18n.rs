use dioxus_translate::*;

translate! {
    SongsTranslate;

    title: {
        ko: "AI와 함께하는 히어로즈 주제곡 공모전",
        en: "Heroes Theme Song Contest with AI",
    },

    description: {
        ko: r#"
※ 인천히어로즈 주제곡 순위가 확정된 이후에도 좋아요를 누르실 수 있습니다!</br>
(6/10 이후 클릭한 좋아요는 최종 순위에 영향을 미치지 않습니다)
"#,
        en: r#"
You can still like the Incheon Heroes theme song even after the final ranking is confirmed!</br>
(Likes clicked after June 10th will not affect the final ranking)
"#,
    }

    results: {
        ko: "results",
        en: "results",
    },
}
