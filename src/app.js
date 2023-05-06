//https://www.youtube.com/watch?v=oMOe_32M6ss

//ICONS
const lightThemeIcon = document.querySelector(".lightTheme");
const darkThemeIcon = document.querySelector(".darkTheme");

//DarkorLightTheme?
// -- User option:
const userTheme = localStorage.getItem("theme");

//Stored in Computer:
const systemTheme = window.matchMedia("(prefers-color-scheme: dark)").matches;

//Icon Toggle
const iconToggle = () => {
    lightThemeIcon.classList.toggle("hidden");
    darkThemeIcon.classList.toggle("hidden");
};

//Initial Theme (based on system preference):
const themeCheck = () => {
    if (userTheme === "dark" || (!userTheme && systemTheme)) {
        document.documentElement.classList.add("dark");
        darkThemeIcon.classList.add("hidden");
        return;
    }
    lightThemeIcon.classList.add("hidden");
};

//Choose theme Manually
const themeChoose = () => {
    if (document.documentElement.classList.contains("dark")) {
        document.documentElement.classList.remove("dark");
        localStorage.setItem("theme", "light");
        iconToggle();
        return;
    }
    document.documentElement.classList.add("dark");
    localStorage.setItem("theme", "dark");
    iconToggle();
};

lightThemeIcon.addEventListener("click", () => {
    themeChoose();
});

darkThemeIcon.addEventListener("click", () => {
    themeChoose();
});

themeCheck();
