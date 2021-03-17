import "./search.js"

const $lang = document.getElementById("lang")

$lang.oninput = () => {
  const lang = $lang.value
  const location = window.location.pathname.replace(/\//g, "\\")
  window.location.pathname = `/api/select-lang?lang=${lang}&location=${location}`
}
