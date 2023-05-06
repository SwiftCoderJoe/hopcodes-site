const colors = require('tailwindcss/colors')

module.exports = {
    content: ['./src/**/*.{js,jsx,ts,tsx,vue,html}'],
    darkMode: 'class',
    theme: {
        colors: {
            sky: colors.sky,
            zinc: colors.zinc,
            maroon: `#8B2832`,
        },
        extend: {
            backgroundImage: {
                'home-icon': `url('/shared/homeIcon.png')`,
                'calendar-icon': `url('/shared/calIcon.png')`,
                'robotics-icon': `url('/shared/roboticsIcon.png')`
            }
        },
    },
    plugins: [],
}