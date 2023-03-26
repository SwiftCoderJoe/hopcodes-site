const colors = require('tailwindcss/colors')

module.exports = {
    content: ['./src/**/*.{js,jsx,ts,tsx,vue,html}'],
    theme: {
        colors: {
            sky: colors.sky,
            slate: colors.slate
        },
        extend: {},
    },
    plugins: [],
}