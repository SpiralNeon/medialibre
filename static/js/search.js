const $input = document.getElementById("search-input")
const $category = document.getElementById("search-category")
const $button = document.getElementById("search-button")

function search() {
  const media = $button.getAttribute("data-media")
  const category = $category.value
  const query = $input.value
  if (query.length) window.location = `/${media}/search/${category}/${query}`
}

$input.onkeyup = (e) => {
  if (e.keyCode === 13) search()
}

$button.onclick = search
