if (window.location.pathname === "/music/artist/new") {
  const $group = document.getElementById("type-group")
  const $person = document.getElementById("type-person")
  const $real_name_label = document.getElementById("real-name-label")
  const $real_name_div = document.getElementById("real-name-div")
  const $formation = document.getElementById("formation")
  const $birth = document.getElementById("birth")
  const $disbandment = document.getElementById("disbandment")
  const $death = document.getElementById("death")
  const $birth_month = document.getElementById("birth-month")
  const $birth_day = document.getElementById("birth-day")
  const $death_month = document.getElementById("death-month")
  const $death_day = document.getElementById("death-day")
  const month_days = JSON.parse(document.querySelector("meta[name=month_days]").content)

  function ongroup() {
  	$real_name_label.classList.add("hidden")
  	$real_name_div.classList.add("hidden")
  	$formation.classList.remove("hidden")
  	$birth.classList.add("hidden")
  	$disbandment.classList.remove("hidden")
  	$death.classList.add("hidden")
  }

  function onperson() {
  	$real_name_label.classList.remove("hidden")
  	$real_name_div.classList.remove("hidden")
  	$formation.classList.add("hidden")
  	$birth.classList.remove("hidden")
  	$disbandment.classList.add("hidden")
  	$death.classList.remove("hidden")
  }

  $group.onclick = ongroup
  $person.onclick = onperson

  if ($group.checked) ongroup()
  else onperson()

  function onmonth(month_el, day_el) {
    const month = parseInt(month_el.value)

    let days_html
    if (month === 0) {
      days_html = ""
    } else {
      const days = [...Array(month_days[month - 1]).keys()]
      days_html = days.map(day => `<option value="${day + 1}">${day + 1}</option>`).join("\n")
    }

    day_el.innerHTML = `
      <option value="0">-- Unknown --</option>
      ${days_html}
    `
  }

  function onbirthmonth() {
    onmonth($birth_month, $birth_day)
  }

  function ondeathmonth() {
    onmonth($death_month, $death_day)
  }

  $birth_month.oninput = onbirthmonth
  $death_month.oninput = ondeathmonth
  onbirthmonth()
  ondeathmonth()
}
