# How the Dashboard JavaScript Works

The dashboard is a single HTML page served by the Rust server at `GET /`. It contains inline JavaScript that calls the same API endpoints you'd use with curl. No frameworks, no build tools — just vanilla JS.

## The Full Flow (Locations Example)

Here's exactly what happens when you click the "Locations" tab:

### Step 1: Browser loads the page

You visit `http://localhost:3000/`. Axum matches this to the route in `src/routes.rs:13`:

```rust
.route("/", get(handlers::dashboard))
```

This calls `src/handlers/dashboard.rs:3-4`, which returns a big HTML string:

```rust
pub async fn dashboard() -> Html<&'static str> {
    Html(DASHBOARD_HTML)
}
```

The browser receives the HTML page and runs the `<script>` block inside it.

### Step 2: You click "Locations"

The HTML has four tab buttons (`dashboard.rs:41-46`):

```html
<div class="tabs">
  <div class="tab active" data-tab="items">Items</div>
  <div class="tab" data-tab="locations">Locations</div>
  <div class="tab" data-tab="trips">Trips</div>
  <div class="tab" data-tab="meals">Meals</div>
</div>
```

Each tab has a `data-tab` attribute — a custom HTML attribute that stores a string (like `"locations"`).

When the page loads, the JS attaches a click listener to every tab (`dashboard.rs:176-183`):

```js
document.querySelectorAll('.tab').forEach(tab => {
  tab.addEventListener('click', () => {
    // Remove "active" styling from all tabs
    document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
    // Add "active" styling to the clicked tab
    tab.classList.add('active');
    // Read the data-tab attribute — e.g. "locations"
    activeTab = tab.dataset.tab;
    // Reset to page 1
    offset = 0;
    // Fetch data from the API
    fetchData();
  });
});
```

So clicking "Locations" sets `activeTab = "locations"` and calls `fetchData()`.

### Step 3: fetchData() builds the URL and calls the API

The `tabConfig` object (`dashboard.rs:69-112`) is a lookup table that maps each tab name to its API endpoint and column definitions:

```js
const tabConfig = {
  items: {
    endpoint: '/items',
    columns: [
      { key: 'name', label: 'Name' },
      { key: 'brand', label: 'Brand' },
      // ... more columns
    ]
  },
  locations: {
    endpoint: '/locations',
    columns: [
      { key: 'name', label: 'Name' },
      { key: 'description', label: 'Description' },
    ]
  },
  // trips, meals...
};
```

`fetchData()` (`dashboard.rs:120-137`) uses `activeTab` to look up the config:

```js
async function fetchData() {
  // activeTab is "locations", so config.endpoint is "/locations"
  const config = tabConfig[activeTab];
  const limit = document.getElementById('perPage').value;  // e.g. "50"
  const search = document.getElementById('search').value.trim();

  // Builds: "/api/v1/locations?limit=50&offset=0&sort=desc"
  let url = `${BASE}${config.endpoint}?limit=${limit}&offset=${offset}&sort=${sortDir}`;
  if (search) url += `&search=${encodeURIComponent(search)}`;

  // Make the HTTP request to your Rust API
  const res = await fetch(url);
  const data = await res.json();

  // Render the results as a table
  renderTable(data, config.columns);
  updatePagination(data.length, parseInt(limit));
}
```

`fetch(url)` is a built-in browser function that makes an HTTP GET request — the same thing curl does, but from within the browser.

### Step 4: Axum handles the API request

The browser's `fetch('/api/v1/locations?limit=50&offset=0&sort=desc')` hits your Rust server.

Axum matches it at `src/routes.rs:24`:

```rust
.route("/locations", get(handlers::list_locations))
```

This calls `src/handlers/locations.rs:13-19`:

```rust
pub async fn list_locations(
    State(pool): State<PgPool>,
    Query(params): Query<ListParams>,  // deserializes ?limit=50&offset=0&sort=desc
) -> AppResult<Json<Vec<Location>>> {
    let locations = db::list_locations(&pool, &params).await?;
    Ok(Json(locations))
}
```

Which calls `src/db/locations.rs:7-33` to build and run the SQL query:

```sql
SELECT id, name, description, created_at, updated_at
FROM locations
ORDER BY name ASC
LIMIT 50 OFFSET 0
```

The result comes back as a JSON array.

### Step 5: renderTable() displays the data

Back in the browser, `renderTable()` (`dashboard.rs:139-161`) loops through the JSON array and builds HTML table rows:

```js
function renderTable(data, columns) {
  let html = '<table><thead><tr>';

  // Build header row from column config
  for (const col of columns) {
    html += `<th>${col.label}</th>`;  // "Name", "Description"
  }
  html += '</tr></thead><tbody>';

  // Build one row per location
  for (const row of data) {
    html += '<tr>';
    for (const col of columns) {
      const val = row[col.key];  // row["name"], row["description"]
      html += `<td>${val}</td>`;
    }
    html += '</tr>';
  }
  html += '</tbody></table>';

  // Insert the table into the page
  document.getElementById('tableContainer').innerHTML = html;
}
```

So if the API returns `[{"name": "pantry", "description": "Main kitchen pantry"}, ...]`, the JS builds:

```html
<table>
  <tr><th>Name</th><th>Description</th></tr>
  <tr><td>pantry</td><td>Main kitchen pantry</td></tr>
  ...
</table>
```

## Search

When you type in the search box, a "debounce" timer waits 300ms after you stop typing, then calls `fetchData()` again (`dashboard.rs:187-190`). The search term gets appended to the URL as `&search=whatever`, which the Rust API handles with an `ILIKE` SQL clause.

## Pagination

The Prev/Next buttons (`dashboard.rs:192-206`) adjust the `offset` variable and call `fetchData()`:

- **Next**: `offset += limit` (e.g., 0 → 50 → 100)
- **Prev**: `offset -= limit` (e.g., 100 → 50 → 0)

The "per page" dropdown changes `limit` (10, 20, or 50).

## Sort

Clicking any column header toggles `sortDir` between `"asc"` and `"desc"` and re-fetches (`dashboard.rs:170-173`).

## Key Concept

The dashboard doesn't know anything about the database. It only knows how to call URLs and render JSON as tables. All the real work (querying, filtering, pagination) happens in the Rust API. The JS is just a thin UI layer.
