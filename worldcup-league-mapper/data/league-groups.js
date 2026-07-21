// Maps a real-world league name (as recorded in players.json) to the
// display "league group" used for browsing. Anything not listed here
// falls into the "Other Leagues" bucket.
const LEAGUE_GROUPS = {
  "Premier League": "Premier League",
  "La Liga": "La Liga",
  "Serie A": "Serie A",
  "Bundesliga": "Bundesliga",
  "Ligue 1": "Ligue 1",
  "MLS": "MLS",
  "Liga MX": "Liga MX",
  "Brasileirão": "Brasileirão",
  "Brasileirão Série A": "Brasileirão",
  "Primeira Liga": "Primeira Liga",
  "Eredivisie": "Eredivisie",
  "Saudi Pro League": "Saudi Pro League",
};

const OTHER_LEAGUE_GROUP = "Other Leagues";

function leagueGroupFor(realLeagueName) {
  return LEAGUE_GROUPS[realLeagueName] || OTHER_LEAGUE_GROUP;
}
