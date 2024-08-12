// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    modules: ['@nuxt/content', "@nuxt/image"],
    css: ['~/assets/css/main.css'],
    devtools: { enabled: true },
    compatibilityDate: '2024-07-27',
    app: {
        pageTransition: { name: "page", mode: "out-in" },
        layoutTransition: { name: "layout", mode: "out-in" }
    },
})
