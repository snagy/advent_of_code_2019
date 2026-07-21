const DATA_URL = "data/players.json";
const FAV_KEY = "wc-league-mapper:favorites";

const state = {
  raw: null,
  byCountry: new Map(),
  byClub: new Map(),
  byLeagueGroup: new Map(),
  countries: [],
  clubs: [],
  leagueGroups: [],
  favorites: loadFavorites(),
};

function loadFavorites() {
  try {
    const parsed = JSON.parse(localStorage.getItem(FAV_KEY) || "{}");
    return { country: parsed.country || {}, club: parsed.club || {} };
  } catch {
    return { country: {}, club: {} };
  }
}

function saveFavorites() {
  localStorage.setItem(FAV_KEY, JSON.stringify(state.favorites));
}

function setFavorite(kind, name, status) {
  const bucket = state.favorites[kind];
  if (bucket[name] === status) {
    delete bucket[name];
  } else {
    bucket[name] = status;
  }
  saveFavorites();
}

function escapeHtml(str) {
  return String(str)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");
}

function slug(name) {
  return encodeURIComponent(name);
}

function buildIndices(data) {
  state.raw = data;
  state.byCountry = new Map();
  state.byClub = new Map();
  state.byLeagueGroup = new Map();

  for (const p of data.players) {
    if (!state.byCountry.has(p.country)) state.byCountry.set(p.country, []);
    state.byCountry.get(p.country).push(p);

    if (!state.byClub.has(p.club)) {
      state.byClub.set(p.club, { league: p.league, clubCountry: p.clubCountry, players: [] });
    }
    state.byClub.get(p.club).players.push(p);

    const group = leagueGroupFor(p.league);
    if (!state.byLeagueGroup.has(group)) state.byLeagueGroup.set(group, new Map());
    const clubMap = state.byLeagueGroup.get(group);
    clubMap.set(p.club, (clubMap.get(p.club) || 0) + 1);
  }

  state.countries = [...state.byCountry.keys()].sort();
  state.clubs = [...state.byClub.keys()].sort();
  state.leagueGroups = [...state.byLeagueGroup.keys()].sort((a, b) => {
    if (a === OTHER_LEAGUE_GROUP) return 1;
    if (b === OTHER_LEAGUE_GROUP) return -1;
    return a.localeCompare(b);
  });
}

function favBtnHtml(kind, name) {
  const current = state.favorites[kind][name];
  return `
    <div class="fav-toggle-group">
      <button class="fav-btn ${current === "fav" ? "active-fav" : ""}" data-fav-kind="${kind}" data-fav-name="${escapeHtml(name)}" data-fav-status="fav" title="Mark as favorite">★ Favorite</button>
      <button class="fav-btn ${current === "unfav" ? "active-unfav" : ""}" data-fav-kind="${kind}" data-fav-name="${escapeHtml(name)}" data-fav-status="unfav" title="Mark as least favorite">✕ Least favorite</button>
    </div>`;
}

function attachFavHandlers(root) {
  root.querySelectorAll("[data-fav-kind]").forEach((btn) => {
    btn.addEventListener("click", () => {
      setFavorite(btn.dataset.favKind, btn.dataset.favName, btn.dataset.favStatus);
      render();
    });
  });
}

function favMarker(kind, name) {
  const s = state.favorites[kind][name];
  if (s === "fav") return `<span class="fav-star" title="Favorite">★</span>`;
  if (s === "unfav") return `<span class="fav-star" title="Least favorite">✕</span>`;
  return `<span class="fav-star"></span>`;
}

function breadcrumbs(parts) {
  const html = parts
    .map((p, i) => (i === parts.length - 1 ? `<span>${p.label}</span>` : `<a href="${p.href}">${p.label}</a>`))
    .join(" &rsaquo; ");
  return `<div class="breadcrumbs">${html}</div>`;
}

function renderHome() {
  const favCountries = Object.keys(state.favorites.country);
  const favClubs = Object.keys(state.favorites.club);

  let favSection = "";
  if (favCountries.length || favClubs.length) {
    favSection = `
      <div class="section-title"><h2>Your favorites</h2></div>
      <div class="chip-grid">
        ${favCountries
          .map(
            (c) =>
              `<a class="chip-card ${state.favorites.country[c] === "fav" ? "is-fav" : "is-unfav"}" href="#/country/${slug(c)}">${escapeHtml(c)} ${favMarker("country", c)}</a>`
          )
          .join("")}
        ${favClubs
          .map(
            (c) =>
              `<a class="chip-card ${state.favorites.club[c] === "fav" ? "is-fav" : "is-unfav"}" href="#/club/${slug(c)}">${escapeHtml(c)} ${favMarker("club", c)}</a>`
          )
          .join("")}
      </div>`;
  }

  return `
    <div class="hero">
      <h1>${escapeHtml(state.raw.tournament)} ↔ Club Leagues</h1>
      <p>See where World Cup players spend their day jobs — and which countries a league's stars represented.</p>
    </div>
    <div class="mode-cards">
      <a class="mode-card" href="#/countries">
        <h2>Browse by Country →</h2>
        <p>Pick a national team and see every player's real-life club and league.</p>
      </a>
      <a class="mode-card" href="#/leagues">
        <h2>Browse by League →</h2>
        <p>Pick a league, drill into a club, and see which countries its World Cup players represented.</p>
      </a>
    </div>
    ${favSection}
  `;
}

function renderCountries() {
  return `
    ${breadcrumbs([{ label: "Home", href: "#/" }, { label: "Countries" }])}
    <div class="section-title"><h2>Countries</h2><span class="count">${state.countries.length}</span></div>
    <div class="chip-grid">
      ${state.countries
        .map(
          (c) =>
            `<a class="chip-card ${favClass("country", c)}" href="#/country/${slug(c)}">${escapeHtml(c)} ${favMarker("country", c)}</a>`
        )
        .join("")}
    </div>
  `;
}

function favClass(kind, name) {
  const s = state.favorites[kind][name];
  return s === "fav" ? "is-fav" : s === "unfav" ? "is-unfav" : "";
}

function renderCountry(name) {
  const players = state.byCountry.get(name);
  if (!players) return renderNotFound(`Country "${name}" not found.`);

  const rows = [...players]
    .sort((a, b) => (a.shirtNumber || 99) - (b.shirtNumber || 99))
    .map(
      (p) => `
      <tr>
        <td>${p.shirtNumber ?? ""}</td>
        <td>${escapeHtml(p.name)}</td>
        <td><span class="pos-badge">${escapeHtml(p.position || "")}</span></td>
        <td><a href="#/club/${slug(p.club)}">${escapeHtml(p.club)}</a></td>
        <td>${escapeHtml(p.league)}</td>
      </tr>`
    )
    .join("");

  return `
    ${breadcrumbs([{ label: "Home", href: "#/" }, { label: "Countries", href: "#/countries" }, { label: escapeHtml(name) }])}
    <div class="page-title">
      <h1>${escapeHtml(name)}</h1>
      ${favBtnHtml("country", name)}
    </div>
    <p class="page-subtitle">${players.length} squad players &mdash; click a club to see who else plays there.</p>
    <table class="roster">
      <thead><tr><th>#</th><th>Player</th><th>Pos</th><th>Club</th><th>League</th></tr></thead>
      <tbody>${rows}</tbody>
    </table>
  `;
}

function renderLeagues() {
  const rows = state.leagueGroups
    .map((g) => {
      const clubs = state.byLeagueGroup.get(g);
      const playerCount = [...clubs.values()].reduce((a, b) => a + b, 0);
      return `
        <a class="league-group-row" href="#/league/${slug(g)}">
          <span>${escapeHtml(g)}</span>
          <span class="meta">${clubs.size} club${clubs.size === 1 ? "" : "s"} &middot; ${playerCount} player${playerCount === 1 ? "" : "s"}</span>
        </a>`;
    })
    .join("");

  return `
    ${breadcrumbs([{ label: "Home", href: "#/" }, { label: "Leagues" }])}
    <div class="section-title"><h2>Leagues</h2><span class="count">${state.leagueGroups.length}</span></div>
    <div class="league-group-list">${rows}</div>
  `;
}

function renderLeague(group) {
  const clubs = state.byLeagueGroup.get(group);
  if (!clubs) return renderNotFound(`League "${group}" not found.`);

  const rows = [...clubs.entries()]
    .sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0]))
    .map(([club, count]) => {
      const info = state.byClub.get(club);
      return `
        <a class="club-row ${favClass("club", club)}" href="#/club/${slug(club)}">
          <span class="club-name">${escapeHtml(club)} ${favMarker("club", club)}</span>
          <span class="club-meta">${count} WC player${count === 1 ? "" : "s"} &middot; ${escapeHtml(info.clubCountry || "")}</span>
        </a>`;
    })
    .join("");

  return `
    ${breadcrumbs([{ label: "Home", href: "#/" }, { label: "Leagues", href: "#/leagues" }, { label: escapeHtml(group) }])}
    <div class="page-title"><h1>${escapeHtml(group)}</h1></div>
    <p class="page-subtitle">Clubs in this league with players who were at the World Cup.</p>
    <div class="club-list">${rows || `<div class="empty-state">No clubs found.</div>`}</div>
  `;
}

function renderClub(name) {
  const info = state.byClub.get(name);
  if (!info) return renderNotFound(`Club "${name}" not found.`);

  const group = leagueGroupFor(info.league);
  const rows = [...info.players]
    .sort((a, b) => a.country.localeCompare(b.country) || a.name.localeCompare(b.name))
    .map(
      (p) => `
      <tr>
        <td>${escapeHtml(p.name)}</td>
        <td><span class="pos-badge">${escapeHtml(p.position || "")}</span></td>
        <td><a href="#/country/${slug(p.country)}">${escapeHtml(p.country)}</a></td>
      </tr>`
    )
    .join("");

  return `
    ${breadcrumbs([
      { label: "Home", href: "#/" },
      { label: "Leagues", href: "#/leagues" },
      { label: escapeHtml(group), href: `#/league/${slug(group)}` },
      { label: escapeHtml(name) },
    ])}
    <div class="page-title">
      <h1>${escapeHtml(name)}</h1>
      ${favBtnHtml("club", name)}
    </div>
    <p class="page-subtitle">${escapeHtml(info.league)} (${escapeHtml(info.clubCountry || "")}) &mdash; ${info.players.length} player${info.players.length === 1 ? "" : "s"} who went to the World Cup, and the countries they represented.</p>
    <table class="roster">
      <thead><tr><th>Player</th><th>Pos</th><th>Country</th></tr></thead>
      <tbody>${rows}</tbody>
    </table>
  `;
}

function renderNotFound(msg) {
  return `${breadcrumbs([{ label: "Home", href: "#/" }])}<div class="empty-state">${escapeHtml(msg)}</div>`;
}

function parseRoute() {
  const hash = location.hash.replace(/^#/, "") || "/";
  const parts = hash.split("/").filter(Boolean);
  return parts;
}

function render() {
  const app = document.getElementById("app");
  const parts = parseRoute();
  let html;

  if (parts.length === 0) {
    html = renderHome();
  } else if (parts[0] === "countries") {
    html = renderCountries();
  } else if (parts[0] === "country" && parts[1]) {
    html = renderCountry(decodeURIComponent(parts[1]));
  } else if (parts[0] === "leagues") {
    html = renderLeagues();
  } else if (parts[0] === "league" && parts[1]) {
    html = renderLeague(decodeURIComponent(parts[1]));
  } else if (parts[0] === "club" && parts[1]) {
    html = renderClub(decodeURIComponent(parts[1]));
  } else {
    html = renderNotFound("Page not found.");
  }

  app.innerHTML = html;
  attachFavHandlers(app);
  window.scrollTo(0, 0);
}

function setupSearch() {
  const input = document.getElementById("search-input");
  const resultsBox = document.getElementById("search-results");

  function search(query) {
    const q = query.trim().toLowerCase();
    if (!q) return [];
    const results = [];

    for (const c of state.countries) {
      if (c.toLowerCase().includes(q)) results.push({ label: c, meta: "Country", href: `#/country/${slug(c)}` });
    }
    for (const c of state.clubs) {
      if (c.toLowerCase().includes(q)) {
        const info = state.byClub.get(c);
        results.push({ label: c, meta: info.league, href: `#/club/${slug(c)}` });
      }
    }
    for (const p of state.raw.players) {
      if (p.name.toLowerCase().includes(q)) {
        results.push({ label: p.name, meta: `${p.country} · ${p.club}`, href: `#/country/${slug(p.country)}` });
      }
    }
    return results.slice(0, 10);
  }

  input.addEventListener("input", () => {
    const results = search(input.value);
    if (!results.length) {
      resultsBox.classList.add("hidden");
      resultsBox.innerHTML = "";
      return;
    }
    resultsBox.innerHTML = results
      .map((r) => `<a href="${r.href}"><span>${escapeHtml(r.label)}</span><span class="muted">${escapeHtml(r.meta)}</span></a>`)
      .join("");
    resultsBox.classList.remove("hidden");
  });

  resultsBox.addEventListener("click", () => {
    input.value = "";
    resultsBox.classList.add("hidden");
    resultsBox.innerHTML = "";
  });

  document.addEventListener("click", (e) => {
    if (!e.target.closest(".search-wrap")) {
      resultsBox.classList.add("hidden");
    }
  });
}

async function init() {
  const res = await fetch(DATA_URL);
  const data = await res.json();
  buildIndices(data);

  document.getElementById("data-note").textContent = data.generatedNote || "";

  setupSearch();
  window.addEventListener("hashchange", render);
  render();
}

init().catch((err) => {
  document.getElementById("app").innerHTML = `<div class="empty-state">Failed to load data: ${escapeHtml(err.message)}<br>If you opened this file directly, try serving it locally: <code>python3 -m http.server</code> from this directory.</div>`;
  console.error(err);
});
