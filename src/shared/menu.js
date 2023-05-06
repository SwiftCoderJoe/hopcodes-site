let navCollapsed = true
let menu = document.getElementById(`navbar-collapsable`)

document.getElementById(`nav-hamburger-button`).addEventListener(`click`, (ev) => {
    if (navCollapsed) {
        menu.classList.remove(`hidden`)
    } else {
        menu.classList.add(`hidden`)
    }

    navCollapsed = !navCollapsed
})