const colors = require('tailwindcss/colors')

module.exports = {
    content: ['./src/**/*.{js,jsx,ts,tsx,vue,html}'],
    theme: {
        colors: {
            sky: colors.sky,
            zinc: colors.zinc,
            maroon: "#8B2832",
            
        },
        extend: {},
    },
    plugins: [],
}