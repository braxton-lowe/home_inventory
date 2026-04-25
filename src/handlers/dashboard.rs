use axum::response::Html;

pub async fn dashboard() -> Html<&'static str> {
    Html(DASHBOARD_HTML)
}

const DASHBOARD_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Home Inventory</title>
<style>
* { box-sizing: border-box; margin: 0; padding: 0; }
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #f5f5f5; color: #333; padding: 20px; max-width: 1200px; margin: 0 auto; }
h1 { margin-bottom: 16px; font-size: 1.5rem; }
.tabs { display: flex; gap: 4px; margin-bottom: 16px; }
.tab { padding: 8px 16px; border: 1px solid #ccc; background: #fff; cursor: pointer; border-radius: 4px 4px 0 0; font-size: 0.9rem; }
.tab.active { background: #4a90d9; color: #fff; border-color: #4a90d9; }
.controls { display: flex; gap: 12px; align-items: center; margin-bottom: 12px; flex-wrap: wrap; }
.controls input { padding: 6px 10px; border: 1px solid #ccc; border-radius: 4px; font-size: 0.9rem; width: 240px; }
.controls select { padding: 6px 8px; border: 1px solid #ccc; border-radius: 4px; font-size: 0.9rem; }
.controls button { padding: 6px 12px; border: 1px solid #ccc; border-radius: 4px; background: #fff; cursor: pointer; font-size: 0.9rem; }
.controls button:disabled { opacity: 0.4; cursor: default; }
.controls button:hover:not(:disabled) { background: #e8e8e8; }
.btn-new { background: #4a90d9 !important; color: #fff !important; border-color: #4a90d9 !important; }
.btn-new:hover { background: #357abd !important; }
.page-info { font-size: 0.85rem; color: #666; }
table { width: 100%; border-collapse: collapse; background: #fff; border-radius: 4px; overflow: hidden; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
th { background: #f0f0f0; text-align: left; padding: 10px 12px; font-size: 0.8rem; text-transform: uppercase; color: #555; cursor: pointer; user-select: none; white-space: nowrap; }
th:hover { background: #e0e0e0; }
th .sort-arrow { margin-left: 4px; font-size: 0.7rem; }
td { padding: 8px 12px; border-top: 1px solid #eee; font-size: 0.85rem; }
tr:hover { background: #f9f9f9; }
.empty { text-align: center; padding: 40px; color: #999; }
a { color: #4a90d9; text-decoration: none; }
a:hover { text-decoration: underline; }
.actions { white-space: nowrap; }
.actions button { background: none; border: none; cursor: pointer; padding: 2px 6px; font-size: 0.85rem; border-radius: 3px; }
.actions button:hover { background: #e8e8e8; }
.btn-edit { color: #4a90d9; }
.btn-del { color: #d94a4a; }
.btn-gone { color: #e08a00; }
.btn-reactivate { color: #48a868; }
.modal-overlay { display: none; position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.4); z-index: 100; justify-content: center; align-items: center; }
.modal-overlay.open { display: flex; }
.modal { background: #fff; border-radius: 8px; padding: 24px; width: 480px; max-width: 90vw; max-height: 90vh; overflow-y: auto; box-shadow: 0 4px 20px rgba(0,0,0,0.2); }
.modal h2 { margin-bottom: 16px; font-size: 1.2rem; }
.modal label { display: block; margin-bottom: 12px; font-size: 0.85rem; color: #555; }
.modal label span { display: block; margin-bottom: 4px; font-weight: 600; }
.modal input, .modal select, .modal textarea { width: 100%; padding: 6px 10px; border: 1px solid #ccc; border-radius: 4px; font-size: 0.9rem; font-family: inherit; }
.modal textarea { resize: vertical; min-height: 60px; }
.modal-btns { display: flex; gap: 8px; justify-content: flex-end; margin-top: 16px; }
.modal-btns button { padding: 8px 16px; border: 1px solid #ccc; border-radius: 4px; cursor: pointer; font-size: 0.9rem; }
.modal-btns .btn-save { background: #4a90d9; color: #fff; border-color: #4a90d9; }
.modal-btns .btn-save:hover { background: #357abd; }
.modal-btns .btn-cancel { background: #fff; }
.modal-btns .btn-cancel:hover { background: #e8e8e8; }
.toast { position: fixed; bottom: 20px; right: 20px; padding: 12px 20px; border-radius: 4px; color: #fff; font-size: 0.9rem; z-index: 200; transition: opacity 0.3s; }
.toast.success { background: #48a868; }
.toast.error { background: #d94a4a; }
.btn-import { background: #5a5a5a !important; color: #fff !important; border-color: #5a5a5a !important; }
.btn-import:hover { background: #444 !important; }
.drop-zone { border: 2px dashed #ccc; border-radius: 8px; padding: 40px 20px; text-align: center; cursor: pointer; color: #999; transition: border-color 0.2s, background 0.2s; }
.drop-zone.dragover { border-color: #4a90d9; background: #e8f0fe; color: #4a90d9; }
.drop-zone.has-file { border-color: #48a868; color: #333; }
.drop-zone input[type="file"] { display: none; }
.import-result { margin-top: 12px; padding: 12px; background: #f0f7f0; border-radius: 4px; font-size: 0.85rem; }
.import-result div { margin-bottom: 4px; }
.suggestions-container { display: grid; gap: 16px; grid-template-columns: repeat(auto-fill, minmax(340px, 1fr)); }
.suggestion-card { background: #fff; border-radius: 8px; padding: 16px; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
.suggestion-card h3 { font-size: 1rem; margin-bottom: 4px; }
.suggestion-card .meta { font-size: 0.8rem; color: #777; margin-bottom: 10px; }
.suggestion-card .meta a { color: #4a90d9; }
.match-bar { height: 8px; border-radius: 4px; background: #eee; margin-bottom: 10px; overflow: hidden; }
.match-bar-fill { height: 100%; border-radius: 4px; transition: width 0.3s; }
.match-pct { font-size: 0.85rem; font-weight: 600; margin-bottom: 6px; }
.ing-list { display: flex; flex-wrap: wrap; gap: 4px; margin-bottom: 6px; }
.ing-tag { padding: 2px 8px; border-radius: 12px; font-size: 0.75rem; }
.ing-tag.matched { background: #e0f5e0; color: #2a7a2a; }
.ing-tag.missing { background: #fce4e4; color: #a33; }
.suggestions-empty { text-align: center; padding: 40px; color: #999; }
.ac-wrapper { position: relative; }
.ac-list { position: absolute; top: 100%; left: 0; right: 0; background: #fff; border: 1px solid #ccc; border-top: none; border-radius: 0 0 4px 4px; max-height: 180px; overflow-y: auto; z-index: 10; display: none; box-shadow: 0 4px 8px rgba(0,0,0,0.1); }
.ac-list.open { display: block; }
.ac-option { padding: 6px 10px; font-size: 0.85rem; cursor: pointer; }
.ac-option:hover, .ac-option.highlighted { background: #e8f0fe; }
.ac-option .ac-sub { color: #888; font-size: 0.75rem; }
.bulk-bar { display: none; align-items: center; gap: 12px; padding: 10px 16px; background: #e8f0fe; border-radius: 4px; margin-bottom: 12px; font-size: 0.9rem; }
.bulk-bar.open { display: flex; }
.bulk-bar .bulk-count { font-weight: 600; }
.bulk-bar button { padding: 6px 12px; border: 1px solid #ccc; border-radius: 4px; cursor: pointer; font-size: 0.85rem; }
.bulk-bar .btn-bulk-gone { background: #e08a00; color: #fff; border-color: #e08a00; }
.bulk-bar .btn-bulk-gone:hover { background: #c77a00; }
.bulk-bar .btn-bulk-reactivate { background: #48a868; color: #fff; border-color: #48a868; }
.bulk-bar .btn-bulk-reactivate:hover { background: #3a8a55; }
.bulk-bar .btn-bulk-cancel { background: #fff; }
.bulk-bar .btn-bulk-cancel:hover { background: #e8e8e8; }
td .row-checkbox, th .select-all { cursor: pointer; width: 16px; height: 16px; }
</style>
</head>
<body>
<h1>Home Inventory</h1>

<div class="tabs">
  <div class="tab active" data-tab="items">Items</div>
  <div class="tab" data-tab="locations">Locations</div>
  <div class="tab" data-tab="trips">Trips</div>
  <div class="tab" data-tab="meals">Meals</div>
  <div class="tab" data-tab="suggestions">What can I make?</div>
</div>

<div class="controls">
  <input type="text" id="search" placeholder="Search...">
  <button class="btn-new" id="newBtn">+ New</button>
  <button class="btn-import" id="importBtn">Import CSV</button>
  <select id="activeFilter">
    <option value="true" selected>Active</option>
    <option value="false">Consumed</option>
    <option value="all">All</option>
  </select>
  <select id="perPage">
    <option value="10">10 per page</option>
    <option value="20">20 per page</option>
    <option value="50" selected>50 per page</option>
  </select>
  <button id="prevBtn" disabled>&laquo; Prev</button>
  <span class="page-info" id="pageInfo"></span>
  <button id="nextBtn">Next &raquo;</button>
</div>

<div class="bulk-bar" id="bulkBar">
  <span class="bulk-count" id="bulkCount">0 selected</span>
  <button class="btn-bulk-gone" id="bulkGoneBtn">Mark Gone</button>
  <button class="btn-bulk-reactivate" id="bulkReactivateBtn">Reactivate</button>
  <button class="btn-bulk-cancel" id="bulkCancelBtn">Clear</button>
</div>
<div id="tableContainer"></div>

<div class="modal-overlay" id="modalOverlay">
  <div class="modal">
    <h2 id="modalTitle"></h2>
    <form id="modalForm"></form>
    <div class="modal-btns">
      <button type="button" class="btn-cancel" id="modalCancel">Cancel</button>
      <button type="button" class="btn-save" id="modalSave">Save</button>
    </div>
  </div>
</div>

<div class="modal-overlay" id="importOverlay">
  <div class="modal">
    <h2 id="importTitle">Import CSV</h2>
    <div class="drop-zone" id="dropZone">
      <input type="file" id="fileInput" accept=".csv">
      <p id="dropText">Drag &amp; drop a CSV file here, or click to browse</p>
    </div>
    <div id="importResult"></div>
    <div class="modal-btns">
      <button type="button" class="btn-cancel" id="importCancel">Cancel</button>
      <button type="button" class="btn-save" id="importUpload" disabled>Upload</button>
    </div>
  </div>
</div>

<script>
const BASE = '/api/v1';
let activeTab = 'items';
let offset = 0;
let sortDir = 'desc';
let sortCol = null;
let searchTimeout = null;
let editingId = null;
let locationsCache = null;
let activeFilter = 'true';

const tabConfig = {
  items: {
    endpoint: '/items',
    importEndpoint: '/import/items',
    label: 'Item',
    columns: [
      { key: 'name', label: 'Name' },
      { key: 'brand', label: 'Brand' },
      { key: 'type', label: 'Type' },
      { key: 'quantity', label: 'Qty' },
      { key: 'unit', label: 'Unit' },
      { key: 'price', label: 'Price', fmt: v => v != null ? '$' + Number(v).toFixed(2) : '' },
      { key: 'purchase_date', label: 'Purchased', fmt: v => v || '' },
      { key: 'expiration_date', label: 'Expires', fmt: v => v || '' },
      { key: 'notes', label: 'Notes' },
    ],
    fields: [
      { key: 'name', label: 'Name', type: 'text', required: true },
      { key: 'type', label: 'Type', type: 'text', required: true, placeholder: 'e.g. produce, dairy, canned' },
      { key: 'brand', label: 'Brand', type: 'text' },
      { key: 'quantity', label: 'Quantity', type: 'number', required: true, step: '0.01' },
      { key: 'unit', label: 'Unit', type: 'text', required: true, placeholder: 'e.g. count, lbs, gallons' },
      { key: 'price', label: 'Price', type: 'number', step: '0.01' },
      { key: 'purchase_date', label: 'Purchase Date', type: 'date', required: true },
      { key: 'expiration_date', label: 'Expiration Date', type: 'date' },
      { key: 'location_id', label: 'Location', type: 'select', required: true },
      { key: 'notes', label: 'Notes', type: 'textarea' },
    ]
  },
  locations: {
    endpoint: '/locations',
    importEndpoint: null,
    label: 'Location',
    columns: [
      { key: 'name', label: 'Name' },
      { key: 'description', label: 'Description' },
    ],
    fields: [
      { key: 'name', label: 'Name', type: 'text', required: true },
      { key: 'description', label: 'Description', type: 'textarea' },
    ]
  },
  trips: {
    endpoint: '/trips',
    importEndpoint: '/import/trips',
    label: 'Trip',
    columns: [
      { key: 'trip_date', label: 'Date' },
      { key: 'store_name', label: 'Store' },
      { key: 'total_spent', label: 'Total', fmt: v => v != null ? '$' + Number(v).toFixed(2) : '' },
      { key: 'notes', label: 'Notes' },
    ],
    fields: [
      { key: 'trip_date', label: 'Date', type: 'date', required: true },
      { key: 'store_name', label: 'Store Name', type: 'text', required: true },
      { key: 'total_spent', label: 'Total Spent', type: 'number', step: '0.01' },
      { key: 'notes', label: 'Notes', type: 'textarea' },
    ]
  },
  meals: {
    endpoint: '/meals',
    importEndpoint: '/import/meals',
    label: 'Meal',
    columns: [
      { key: 'name', label: 'Name' },
      { key: 'made_on', label: 'Made On' },
      { key: 'servings', label: 'Servings' },
      { key: 'rating', label: 'Rating' },
      { key: 'ingredients', label: 'Ingredients' },
      { key: 'recipe_link', label: 'Recipe', fmt: v => v ? '<a href="' + esc(v) + '" target="_blank">Link</a>' : '' },
      { key: 'last_eaten', label: 'Last Eaten', fmt: v => v || '' },
    ],
    fields: [
      { key: 'name', label: 'Name', type: 'text', required: true },
      { key: 'made_on', label: 'Made On', type: 'date', required: true },
      { key: 'servings', label: 'Servings', type: 'number', required: true, step: '1' },
      { key: 'rating', label: 'Rating', type: 'text', required: true, placeholder: 'e.g. loved, enjoyed, okay, disliked' },
      { key: 'ingredients', label: 'Ingredients', type: 'textarea' },
      { key: 'recipe_link', label: 'Recipe Link', type: 'text' },
      { key: 'last_eaten', label: 'Last Eaten', type: 'date' },
    ]
  }
};

function esc(s) {
  const d = document.createElement('div');
  d.textContent = s;
  return d.innerHTML;
}

function showToast(msg, type) {
  const el = document.createElement('div');
  el.className = 'toast ' + type;
  el.textContent = msg;
  document.body.appendChild(el);
  setTimeout(() => { el.style.opacity = '0'; setTimeout(() => el.remove(), 300); }, 2000);
}

async function fetchLocations() {
  if (locationsCache) return locationsCache;
  const res = await fetch(`${BASE}/locations?limit=100&sort=asc`);
  if (!res.ok) return [];
  locationsCache = await res.json();
  return locationsCache;
}

async function fetchData() {
  const config = tabConfig[activeTab];
  const limit = document.getElementById('perPage').value;
  const search = document.getElementById('search').value.trim();
  let url = `${BASE}${config.endpoint}?limit=${limit}&offset=${offset}&sort=${sortDir}`;
  if (sortCol) url += `&sort_by=${encodeURIComponent(sortCol)}`;
  if (search) url += `&search=${encodeURIComponent(search)}`;
  if (activeTab === 'items') url += `&active=${activeFilter}`;

  try {
    const res = await fetch(url);
    if (!res.ok) throw new Error(res.statusText);
    const data = await res.json();
    let cols = config.columns;
    if (activeTab === 'items' && activeFilter !== 'true') {
      cols = [...cols, { key: 'consumed_at', label: 'Consumed At', fmt: v => v ? new Date(v).toLocaleDateString() : '' }];
    }
    renderTable(data, cols);
    updatePagination(data.length, parseInt(limit));
  } catch (e) {
    document.getElementById('tableContainer').innerHTML =
      '<div class="empty">Error loading data: ' + esc(e.message) + '</div>';
  }
}

function renderTable(data, columns) {
  if (data.length === 0) {
    document.getElementById('tableContainer').innerHTML = '<div class="empty">No results found.</div>';
    return;
  }
  const showCheckbox = activeTab === 'items';
  const arrow = sortDir === 'asc' ? '&#9650;' : '&#9660;';
  let html = '<table><thead><tr>';
  if (showCheckbox) {
    html += '<th style="width:32px"><input type="checkbox" class="select-all" onchange="toggleSelectAll(this.checked)"></th>';
  }
  for (const col of columns) {
    const active = sortCol === col.key;
    html += `<th onclick="sortBy('${col.key}')">${esc(col.label)}${active ? '<span class="sort-arrow">' + arrow + '</span>' : ''}</th>`;
  }
  html += '<th>Actions</th>';
  html += '</tr></thead><tbody>';
  for (const row of data) {
    html += '<tr>';
    if (showCheckbox) {
      const checked = selectedIds.has(row.id) ? ' checked' : '';
      html += `<td><input type="checkbox" class="row-checkbox" data-id="${row.id}"${checked} onchange="toggleSelectRow('${row.id}', this.checked)"></td>`;
    }
    for (const col of columns) {
      const val = row[col.key];
      const display = col.fmt ? col.fmt(val) : (val != null ? esc(String(val)) : '');
      html += `<td>${display}</td>`;
    }
    html += `<td class="actions">`;
    html += `<button class="btn-edit" onclick="openEditModal('${row.id}')" title="Edit">&#9998;</button>`;
    if (activeTab === 'items') {
      if (row.active === false) {
        html += `<button class="btn-reactivate" onclick="reactivateItem('${row.id}')" title="Reactivate">&#8634;</button>`;
      } else {
        html += `<button class="btn-gone" onclick="markGone('${row.id}')" title="Mark Gone">&#10005;</button>`;
      }
    } else {
      html += `<button class="btn-del" onclick="deleteRow('${row.id}')" title="Delete">&#128465;</button>`;
    }
    html += `</td>`;
    html += '</tr>';
  }
  html += '</tbody></table>';
  document.getElementById('tableContainer').innerHTML = html;
}

function updatePagination(count, limit) {
  const page = Math.floor(offset / limit) + 1;
  document.getElementById('pageInfo').textContent = `Page ${page}`;
  document.getElementById('prevBtn').disabled = offset === 0;
  document.getElementById('nextBtn').disabled = count < limit;
}

function sortBy(col) {
  if (sortCol === col) {
    sortDir = sortDir === 'asc' ? 'desc' : 'asc';
  } else {
    sortCol = col;
    sortDir = 'asc';
  }
  offset = 0;
  fetchData();
}

// --- Modal CRUD ---

async function buildForm(fields, data) {
  const form = document.getElementById('modalForm');
  form.innerHTML = '';

  let locations = [];
  if (fields.some(f => f.type === 'select' && f.key === 'location_id')) {
    locations = await fetchLocations();
  }

  for (const field of fields) {
    const label = document.createElement('label');
    const span = document.createElement('span');
    span.textContent = field.label + (field.required ? ' *' : '');
    label.appendChild(span);

    let input;
    if (field.type === 'textarea') {
      input = document.createElement('textarea');
    } else if (field.type === 'select' && field.key === 'location_id') {
      input = document.createElement('select');
      const empty = document.createElement('option');
      empty.value = '';
      empty.textContent = '-- Select location --';
      input.appendChild(empty);
      for (const loc of locations) {
        const opt = document.createElement('option');
        opt.value = loc.id;
        opt.textContent = loc.name;
        input.appendChild(opt);
      }
    } else {
      input = document.createElement('input');
      input.type = field.type || 'text';
      if (field.step) input.step = field.step;
      if (field.placeholder) input.placeholder = field.placeholder;
    }

    input.name = field.key;
    if (field.required) input.required = true;

    if (data && data[field.key] != null) {
      input.value = data[field.key];
    }

    label.appendChild(input);
    form.appendChild(label);
  }
}

async function openCreateModal() {
  editingId = null;
  const config = tabConfig[activeTab];
  document.getElementById('modalTitle').textContent = 'New ' + config.label;
  await buildForm(config.fields, null);
  document.getElementById('modalOverlay').classList.add('open');
  setupAutocomplete();
}

async function openEditModal(id) {
  editingId = id;
  const config = tabConfig[activeTab];
  document.getElementById('modalTitle').textContent = 'Edit ' + config.label;

  try {
    const res = await fetch(`${BASE}${config.endpoint}/${id}`);
    if (!res.ok) throw new Error(res.statusText);
    const data = await res.json();
    await buildForm(config.fields, data);
    document.getElementById('modalOverlay').classList.add('open');
    setupAutocomplete();
  } catch (e) {
    showToast('Error loading record: ' + e.message, 'error');
  }
}

function closeModal() {
  document.getElementById('modalOverlay').classList.remove('open');
  editingId = null;
}

async function saveModal() {
  const config = tabConfig[activeTab];
  const form = document.getElementById('modalForm');

  if (!form.checkValidity()) {
    form.reportValidity();
    return;
  }

  const body = {};
  for (const field of config.fields) {
    const input = form.querySelector(`[name="${field.key}"]`);
    const val = input.value.trim();
    if (val === '') {
      body[field.key] = null;
    } else if (field.type === 'number') {
      body[field.key] = Number(val);
    } else {
      body[field.key] = val;
    }
  }

  const method = editingId ? 'PUT' : 'POST';
  const url = editingId
    ? `${BASE}${config.endpoint}/${editingId}`
    : `${BASE}${config.endpoint}`;

  try {
    const res = await fetch(url, {
      method,
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!res.ok) {
      const err = await res.text();
      throw new Error(err);
    }
    closeModal();
    if (activeTab === 'locations') locationsCache = null;
    showToast(editingId ? 'Updated!' : 'Created!', 'success');
    fetchData();
  } catch (e) {
    showToast('Error: ' + e.message, 'error');
  }
}

async function deleteRow(id) {
  const config = tabConfig[activeTab];
  if (!confirm(`Delete this ${config.label.toLowerCase()}?`)) return;

  try {
    const res = await fetch(`${BASE}${config.endpoint}/${id}`, { method: 'DELETE' });
    if (!res.ok) {
      const err = await res.text();
      throw new Error(err);
    }
    if (activeTab === 'locations') locationsCache = null;
    showToast('Deleted!', 'success');
    fetchData();
  } catch (e) {
    showToast('Error: ' + e.message, 'error');
  }
}

async function markGone(id) {
  if (!confirm('Mark this item as gone?')) return;
  try {
    const res = await fetch(`${BASE}/items/${id}`, { method: 'DELETE' });
    if (!res.ok) {
      const err = await res.text();
      throw new Error(err);
    }
    showToast('Marked as gone', 'success');
    fetchData();
  } catch (e) {
    showToast('Error: ' + e.message, 'error');
  }
}

async function reactivateItem(id) {
  try {
    const res = await fetch(`${BASE}/items/${id}/reactivate`, { method: 'PUT' });
    if (!res.ok) {
      const err = await res.text();
      throw new Error(err);
    }
    showToast('Reactivated!', 'success');
    fetchData();
  } catch (e) {
    showToast('Error: ' + e.message, 'error');
  }
}

// --- CSV Import ---

let importFile = null;

function updateImportBtnVisibility() {
  const config = tabConfig[activeTab];
  document.getElementById('importBtn').style.display = config.importEndpoint ? '' : 'none';
}

function updateActiveFilterVisibility() {
  document.getElementById('activeFilter').style.display = activeTab === 'items' ? '' : 'none';
}

function openImportModal() {
  importFile = null;
  const config = tabConfig[activeTab];
  document.getElementById('importTitle').textContent = 'Import ' + config.label + 's CSV';
  document.getElementById('dropText').textContent = 'Drag & drop a CSV file here, or click to browse';
  document.getElementById('dropZone').classList.remove('has-file');
  document.getElementById('importUpload').disabled = true;
  document.getElementById('importResult').innerHTML = '';
  document.getElementById('fileInput').value = '';
  document.getElementById('importOverlay').classList.add('open');
}

function closeImportModal() {
  document.getElementById('importOverlay').classList.remove('open');
  importFile = null;
}

function handleImportFile(file) {
  if (!file || !file.name.endsWith('.csv')) {
    showToast('Please select a .csv file', 'error');
    return;
  }
  importFile = file;
  document.getElementById('dropText').textContent = file.name;
  document.getElementById('dropZone').classList.add('has-file');
  document.getElementById('importUpload').disabled = false;
}

async function uploadCsv() {
  if (!importFile) return;
  const config = tabConfig[activeTab];
  const formData = new FormData();
  formData.append('file', importFile);

  document.getElementById('importUpload').disabled = true;
  document.getElementById('importUpload').textContent = 'Uploading...';

  try {
    const res = await fetch(`${BASE}${config.importEndpoint}`, {
      method: 'POST',
      body: formData,
    });
    if (!res.ok) {
      const err = await res.text();
      throw new Error(err);
    }
    const result = await res.json();
    let html = '<div class="import-result">';
    html += `<div><strong>Rows processed:</strong> ${result.rows_processed}</div>`;
    for (const [key, val] of Object.entries(result)) {
      if (key !== 'rows_processed' && val != null) {
        const label = key.replace(/_/g, ' ').replace(/\b\w/g, c => c.toUpperCase());
        html += `<div><strong>${esc(label)}:</strong> ${val}</div>`;
      }
    }
    html += '</div>';
    document.getElementById('importResult').innerHTML = html;
    if (activeTab === 'locations') locationsCache = null;
    showToast('Import successful!', 'success');
    fetchData();
  } catch (e) {
    showToast('Import error: ' + e.message, 'error');
  }

  document.getElementById('importUpload').disabled = false;
  document.getElementById('importUpload').textContent = 'Upload';
}

// --- Suggestions ---

async function fetchSuggestions() {
  const container = document.getElementById('tableContainer');
  container.innerHTML = '<div class="suggestions-empty">Loading suggestions...</div>';
  try {
    const res = await fetch(`${BASE}/meals/suggestions`);
    if (!res.ok) throw new Error(res.statusText);
    const data = await res.json();
    renderSuggestions(data);
  } catch (e) {
    container.innerHTML = '<div class="suggestions-empty">Error loading suggestions: ' + esc(e.message) + '</div>';
  }
}

function renderSuggestions(data) {
  const container = document.getElementById('tableContainer');
  if (data.length === 0) {
    container.innerHTML = '<div class="suggestions-empty">No meal suggestions — add meals with ingredients and stock some items!</div>';
    return;
  }
  let html = '<div class="suggestions-container">';
  for (const s of data) {
    const pct = s.match_percentage;
    const barColor = pct >= 75 ? '#48a868' : pct >= 40 ? '#e0a030' : '#d94a4a';
    html += '<div class="suggestion-card">';
    html += `<h3>${esc(s.meal.name)}</h3>`;
    let meta = `${esc(s.meal.rating)}`;
    if (s.meal.servings) meta += ` · ${s.meal.servings} servings`;
    if (s.meal.recipe_link) meta += ` · <a href="${esc(s.meal.recipe_link)}" target="_blank">Recipe</a>`;
    if (s.meal.last_eaten) meta += ` · Last eaten: ${s.meal.last_eaten}`;
    html += `<div class="meta">${meta}</div>`;
    html += `<div class="match-pct" style="color:${barColor}">${pct}% match</div>`;
    html += `<div class="match-bar"><div class="match-bar-fill" style="width:${pct}%;background:${barColor}"></div></div>`;
    if (s.matched_ingredients.length) {
      html += '<div class="ing-list">';
      for (const ing of s.matched_ingredients) html += `<span class="ing-tag matched">${esc(ing)}</span>`;
      html += '</div>';
    }
    if (s.missing_ingredients.length) {
      html += '<div class="ing-list">';
      for (const ing of s.missing_ingredients) html += `<span class="ing-tag missing">${esc(ing)}</span>`;
      html += '</div>';
    }
    html += '</div>';
  }
  html += '</div>';
  container.innerHTML = html;
}

// --- Bulk Selection ---

let selectedIds = new Set();

function updateBulkBar() {
  const bar = document.getElementById('bulkBar');
  if (activeTab !== 'items' || selectedIds.size === 0) {
    bar.classList.remove('open');
    return;
  }
  bar.classList.add('open');
  document.getElementById('bulkCount').textContent = selectedIds.size + ' selected';
  // Show the right button based on current filter
  document.getElementById('bulkGoneBtn').style.display = activeFilter === 'false' ? 'none' : '';
  document.getElementById('bulkReactivateBtn').style.display = activeFilter === 'true' ? 'none' : '';
}

function clearSelection() {
  selectedIds.clear();
  document.querySelectorAll('.row-checkbox').forEach(cb => cb.checked = false);
  const sa = document.querySelector('.select-all');
  if (sa) sa.checked = false;
  updateBulkBar();
}

function toggleSelectAll(checked) {
  document.querySelectorAll('.row-checkbox').forEach(cb => {
    cb.checked = checked;
    const id = cb.dataset.id;
    if (checked) selectedIds.add(id);
    else selectedIds.delete(id);
  });
  updateBulkBar();
}

function toggleSelectRow(id, checked) {
  if (checked) selectedIds.add(id);
  else selectedIds.delete(id);
  // Update select-all state
  const all = document.querySelectorAll('.row-checkbox');
  const sa = document.querySelector('.select-all');
  if (sa) sa.checked = all.length > 0 && [...all].every(cb => cb.checked);
  updateBulkBar();
}

async function bulkMarkGone() {
  if (selectedIds.size === 0) return;
  if (!confirm(`Mark ${selectedIds.size} item(s) as gone?`)) return;
  try {
    const res = await fetch(`${BASE}/items/bulk/deactivate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ ids: [...selectedIds] }),
    });
    if (!res.ok) throw new Error(await res.text());
    const result = await res.json();
    showToast(`${result.affected} item(s) marked as gone`, 'success');
    clearSelection();
    fetchData();
  } catch (e) {
    showToast('Error: ' + e.message, 'error');
  }
}

async function bulkReactivate() {
  if (selectedIds.size === 0) return;
  if (!confirm(`Reactivate ${selectedIds.size} item(s)?`)) return;
  try {
    const res = await fetch(`${BASE}/items/bulk/reactivate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ ids: [...selectedIds] }),
    });
    if (!res.ok) throw new Error(await res.text());
    const result = await res.json();
    showToast(`${result.affected} item(s) reactivated`, 'success');
    clearSelection();
    fetchData();
  } catch (e) {
    showToast('Error: ' + e.message, 'error');
  }
}

// --- Autocomplete ---

let acTimeout = null;
let acHighlight = -1;
let acResults = [];

function setupAutocomplete() {
  // Only for items tab
  if (activeTab !== 'items') return;
  const nameInput = document.querySelector('#modalForm [name="name"]');
  if (!nameInput) return;

  // Wrap the input in a relative container
  const wrapper = document.createElement('div');
  wrapper.className = 'ac-wrapper';
  nameInput.parentNode.insertBefore(wrapper, nameInput);
  wrapper.appendChild(nameInput);

  const list = document.createElement('div');
  list.className = 'ac-list';
  list.id = 'acList';
  wrapper.appendChild(list);

  nameInput.addEventListener('input', () => {
    clearTimeout(acTimeout);
    const q = nameInput.value.trim();
    if (q.length < 2) { closeAc(); return; }
    acTimeout = setTimeout(() => fetchAc(q), 200);
  });

  nameInput.addEventListener('keydown', (e) => {
    if (!list.classList.contains('open')) return;
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      acHighlight = Math.min(acHighlight + 1, acResults.length - 1);
      renderAcHighlight();
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      acHighlight = Math.max(acHighlight - 1, 0);
      renderAcHighlight();
    } else if (e.key === 'Enter' && acHighlight >= 0) {
      e.preventDefault();
      selectAc(acResults[acHighlight]);
    } else if (e.key === 'Escape') {
      closeAc();
    }
  });

  // Close on outside click
  document.addEventListener('click', (e) => {
    if (!wrapper.contains(e.target)) closeAc();
  }, { once: false });
}

async function fetchAc(q) {
  try {
    const res = await fetch(`${BASE}/items/autocomplete?q=${encodeURIComponent(q)}`);
    if (!res.ok) return;
    acResults = await res.json();
    acHighlight = -1;
    renderAc();
  } catch (e) { /* ignore */ }
}

function renderAc() {
  const list = document.getElementById('acList');
  if (!list || acResults.length === 0) { closeAc(); return; }
  list.innerHTML = '';
  for (let i = 0; i < acResults.length; i++) {
    const item = acResults[i];
    const div = document.createElement('div');
    div.className = 'ac-option';
    let label = esc(item.name);
    const sub = [item.type, item.brand, item.unit].filter(Boolean).join(' · ');
    if (sub) label += ` <span class="ac-sub">${esc(sub)}</span>`;
    div.innerHTML = label;
    div.addEventListener('mousedown', (e) => { e.preventDefault(); selectAc(item); });
    list.appendChild(div);
  }
  list.classList.add('open');
}

function renderAcHighlight() {
  const list = document.getElementById('acList');
  if (!list) return;
  const opts = list.querySelectorAll('.ac-option');
  opts.forEach((o, i) => o.classList.toggle('highlighted', i === acHighlight));
  if (opts[acHighlight]) opts[acHighlight].scrollIntoView({ block: 'nearest' });
}

function selectAc(item) {
  const form = document.getElementById('modalForm');
  // Fill in all matching fields from the selected past item
  const fieldMap = {
    name: item.name,
    type: item.type || item.item_type,
    brand: item.brand,
    unit: item.unit,
    price: item.price,
    location_id: item.location_id,
    notes: item.notes,
  };
  for (const [key, val] of Object.entries(fieldMap)) {
    const input = form.querySelector(`[name="${key}"]`);
    if (input && val != null) input.value = val;
  }
  closeAc();
}

function closeAc() {
  const list = document.getElementById('acList');
  if (list) list.classList.remove('open');
  acResults = [];
  acHighlight = -1;
}

// --- Event Listeners ---

// Tab clicks
document.querySelectorAll('.tab').forEach(tab => {
  tab.addEventListener('click', () => {
    document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
    tab.classList.add('active');
    activeTab = tab.dataset.tab;
    offset = 0;
    sortCol = null;
    sortDir = 'desc';
    activeFilter = 'true';
    document.getElementById('activeFilter').value = 'true';
    clearSelection();
    if (activeTab === 'suggestions') {
      document.querySelector('.controls').style.display = 'none';
      fetchSuggestions();
    } else {
      document.querySelector('.controls').style.display = '';
      updateImportBtnVisibility();
      updateActiveFilterVisibility();
      fetchData();
    }
  });
});

// Search with debounce
document.getElementById('search').addEventListener('input', () => {
  clearTimeout(searchTimeout);
  searchTimeout = setTimeout(() => { offset = 0; fetchData(); }, 300);
});

// Pagination
document.getElementById('prevBtn').addEventListener('click', () => {
  const limit = parseInt(document.getElementById('perPage').value);
  offset = Math.max(0, offset - limit);
  fetchData();
});
document.getElementById('nextBtn').addEventListener('click', () => {
  const limit = parseInt(document.getElementById('perPage').value);
  offset += limit;
  fetchData();
});
document.getElementById('perPage').addEventListener('change', () => {
  offset = 0;
  fetchData();
});

// Active filter
document.getElementById('activeFilter').addEventListener('change', (e) => {
  activeFilter = e.target.value;
  offset = 0;
  fetchData();
});

// New button
document.getElementById('newBtn').addEventListener('click', openCreateModal);

// Modal buttons
document.getElementById('modalCancel').addEventListener('click', closeModal);
document.getElementById('modalSave').addEventListener('click', saveModal);
document.getElementById('modalOverlay').addEventListener('click', (e) => {
  if (e.target === document.getElementById('modalOverlay')) closeModal();
});

// Bulk action buttons
document.getElementById('bulkGoneBtn').addEventListener('click', bulkMarkGone);
document.getElementById('bulkReactivateBtn').addEventListener('click', bulkReactivate);
document.getElementById('bulkCancelBtn').addEventListener('click', clearSelection);

// Import button
document.getElementById('importBtn').addEventListener('click', openImportModal);
document.getElementById('importCancel').addEventListener('click', closeImportModal);
document.getElementById('importUpload').addEventListener('click', uploadCsv);
document.getElementById('importOverlay').addEventListener('click', (e) => {
  if (e.target === document.getElementById('importOverlay')) closeImportModal();
});

// Drop zone
const dropZone = document.getElementById('dropZone');
dropZone.addEventListener('click', () => document.getElementById('fileInput').click());
dropZone.addEventListener('dragover', (e) => { e.preventDefault(); dropZone.classList.add('dragover'); });
dropZone.addEventListener('dragleave', () => dropZone.classList.remove('dragover'));
dropZone.addEventListener('drop', (e) => {
  e.preventDefault();
  dropZone.classList.remove('dragover');
  if (e.dataTransfer.files.length) handleImportFile(e.dataTransfer.files[0]);
});
document.getElementById('fileInput').addEventListener('change', (e) => {
  if (e.target.files.length) handleImportFile(e.target.files[0]);
});

// Initial load
updateImportBtnVisibility();
updateActiveFilterVisibility();
fetchData();
</script>
</body>
</html>
"##;
