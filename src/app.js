//https://www.youtube.com/watch?v=oMOe_32M6ss

//ICONS
const lightThemeIcons = document.querySelectorAll(".lightTheme");
const darkThemeIcons = document.querySelectorAll(".darkTheme");

//Initial Theme (based on system preference):
const setTheme = () => {
    if (localStorage.theme === 'dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
        document.documentElement.classList.add('dark')
        lightThemeIcons.forEach((item) => item.classList.remove('hidden'))
        darkThemeIcons.forEach((item) => item.classList.add('hidden'))
    } else {
        document.documentElement.classList.remove('dark')
        lightThemeIcons.forEach((item) => item.classList.add('hidden'))
        darkThemeIcons.forEach((item) => item.classList.remove('hidden'))
    }
};

lightThemeIcons.forEach((item) => item.addEventListener("click", () => {
    localStorage.theme = 'light'
    setTheme();
}));

darkThemeIcons.forEach((item) => item.addEventListener("click", () => {
    localStorage.theme = 'dark'
    setTheme();
}));

setTheme();
