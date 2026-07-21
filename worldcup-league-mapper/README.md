# World Cup ↔ League Mapper

A static website that maps 2026 FIFA World Cup national team rosters to the
club leagues those players work in day-to-day, and back again.

- **Browse by Country** → pick a national team → see each player's real club
  and league → click a club to jump to it.
- **Browse by League** → pick a league (Premier League, La Liga, Serie A,
  Bundesliga, Ligue 1, and a few others; everything else falls under "Other
  Leagues") → pick a club → see which countries its World Cup players
  represented → click a country to jump to its full roster.
- Mark countries/clubs as a favorite (★) or least favorite (✕) for quick
  access from the home page.
- Search box in the header finds players, clubs, or countries directly.

## Running locally

Data is loaded via `fetch`, which most browsers block for pages opened
directly from disk (`file://`). Serve the directory instead:

```sh
cd worldcup-league-mapper
python3 -m http.server 8000
```

Then open http://localhost:8000/.

## Data

`data/players.json` holds every player's name, position, shirt number,
country, club, club country, and real league name. `data/league-groups.js`
maps real league names to the display "league group" buckets used for
browsing (`Premier League`, `La Liga`, etc., with anything unmapped
grouped under "Other Leagues") — edit that file to add more explicit
league groups.
