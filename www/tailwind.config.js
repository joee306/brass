/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/pages/*.{html, js, astro}",
        "./src/layouts/*.{html, js, astro}",
        "./src/components/*.{html, js, astro}",
        "./base.css",
        "./src/**/*"
    ],
    theme: {
        extend: {},
    },
    plugins: [require("tailwindcss")],
}

