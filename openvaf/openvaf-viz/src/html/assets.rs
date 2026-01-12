//! Embedded CSS and JavaScript assets

/// CSS styles for the visualization
pub const CSS: &str = r#"
:root {
    --bg-primary: #1e1e1e;
    --bg-secondary: #252526;
    --bg-tertiary: #2d2d30;
    --text-primary: #d4d4d4;
    --text-secondary: #9cdcfe;
    --text-muted: #808080;
    --accent: #569cd6;
    --accent-secondary: #4ec9b0;
    --border: #3c3c3c;
    --success: #4caf50;
    --warning: #ff9800;
    --error: #f44336;
}

* {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

body {
    font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
    font-size: 13px;
    background: var(--bg-primary);
    color: var(--text-primary);
    line-height: 1.5;
}

.container {
    display: flex;
    height: 100vh;
}

.sidebar {
    width: 300px;
    background: var(--bg-secondary);
    border-right: 1px solid var(--border);
    overflow-y: auto;
    flex-shrink: 0;
}

.main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
}

.header {
    padding: 12px 16px;
    background: var(--bg-tertiary);
    border-bottom: 1px solid var(--border);
}

.header h1 {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-secondary);
}

.tabs {
    display: flex;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border);
}

.tab {
    padding: 8px 16px;
    cursor: pointer;
    border: none;
    background: transparent;
    color: var(--text-muted);
    font-family: inherit;
    font-size: 13px;
    border-bottom: 2px solid transparent;
}

.tab:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
}

.tab.active {
    color: var(--accent);
    border-bottom-color: var(--accent);
}

.panel {
    flex: 1;
    overflow: auto;
    padding: 16px;
}

.panel-hidden {
    display: none;
}

/* Sidebar sections */
.section {
    border-bottom: 1px solid var(--border);
}

.section-header {
    padding: 10px 16px;
    cursor: pointer;
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--bg-tertiary);
}

.section-header:hover {
    background: var(--bg-primary);
}

.section-header h3 {
    font-size: 12px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--text-muted);
}

.section-content {
    padding: 8px 0;
}

.section-content.collapsed {
    display: none;
}

/* Tree items */
.tree-item {
    padding: 4px 16px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 8px;
}

.tree-item:hover {
    background: var(--bg-tertiary);
}

.tree-item.selected {
    background: var(--accent);
    background-opacity: 0.2;
}

.tree-item .icon {
    width: 16px;
    text-align: center;
    color: var(--text-muted);
}

/* Stats */
.stats {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;
    padding: 16px;
}

.stat {
    background: var(--bg-tertiary);
    padding: 12px;
    border-radius: 4px;
}

.stat-value {
    font-size: 24px;
    font-weight: 600;
    color: var(--accent-secondary);
}

.stat-label {
    font-size: 11px;
    color: var(--text-muted);
    text-transform: uppercase;
}

/* CFG visualization */
#cfg-container {
    width: 100%;
    height: 100%;
    background: var(--bg-primary);
}

/* Instruction list */
.block {
    margin-bottom: 16px;
    background: var(--bg-secondary);
    border-radius: 4px;
    overflow: hidden;
}

.block-header {
    padding: 8px 12px;
    background: var(--bg-tertiary);
    display: flex;
    justify-content: space-between;
    align-items: center;
    cursor: pointer;
}

.block-header:hover {
    background: var(--bg-primary);
}

.block-name {
    color: var(--accent);
    font-weight: 600;
}

.block-meta {
    color: var(--text-muted);
    font-size: 11px;
}

.block-content {
    padding: 8px 0;
}

.instruction {
    padding: 4px 12px;
    display: flex;
    gap: 12px;
    font-family: inherit;
}

.instruction:hover {
    background: var(--bg-tertiary);
}

.inst-result {
    color: var(--accent-secondary);
    min-width: 60px;
}

.inst-opcode {
    color: var(--accent);
    min-width: 80px;
}

.inst-args {
    color: var(--text-primary);
}

.inst-opcode.arithmetic { color: #ce9178; }
.inst-opcode.math { color: #b5cea8; }
.inst-opcode.comparison { color: #c586c0; }
.inst-opcode.control_flow { color: #569cd6; }
.inst-opcode.phi { color: #4ec9b0; }
.inst-opcode.call { color: #dcdcaa; }

/* Value references */
.value-ref {
    color: var(--accent-secondary);
    cursor: pointer;
    text-decoration: underline;
    text-decoration-style: dotted;
}

.value-ref:hover {
    text-decoration-style: solid;
}

/* Search */
.search-box {
    padding: 8px 16px;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border);
}

.search-input {
    width: 100%;
    padding: 8px 12px;
    background: var(--bg-primary);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    font-family: inherit;
    font-size: 13px;
}

.search-input:focus {
    outline: none;
    border-color: var(--accent);
}

/* Tooltip */
.tooltip {
    position: fixed;
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 8px 12px;
    max-width: 400px;
    z-index: 1000;
    box-shadow: 0 4px 12px rgba(0,0,0,0.3);
}

.tooltip-title {
    color: var(--accent-secondary);
    font-weight: 600;
    margin-bottom: 4px;
}

.tooltip-content {
    color: var(--text-primary);
    font-size: 12px;
}

/* Pre/code for text summary */
pre {
    background: var(--bg-secondary);
    padding: 16px;
    border-radius: 4px;
    overflow-x: auto;
    white-space: pre-wrap;
    word-wrap: break-word;
}

/* Graph visualization */
.graph-container {
    width: 100%;
    height: 100%;
    overflow: hidden;
    position: relative;
}

.graph-controls {
    position: absolute;
    top: 10px;
    right: 10px;
    display: flex;
    gap: 8px;
    z-index: 100;
}

.graph-btn {
    padding: 6px 12px;
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    cursor: pointer;
    font-family: inherit;
    font-size: 12px;
}

.graph-btn:hover {
    background: var(--bg-secondary);
    border-color: var(--accent);
}

#cfg-graph {
    width: 100%;
    height: 100%;
}

#cfg-graph svg {
    width: 100%;
    height: 100%;
}

/* Graph node styles */
.node rect {
    stroke: var(--border);
    stroke-width: 2px;
    rx: 4;
    ry: 4;
}

.node.entry rect {
    fill: #2d5a3d;
    stroke: #4caf50;
}

.node.exit rect {
    fill: #5a2d3d;
    stroke: #f44336;
}

.node.loop-header rect {
    fill: #5a4d2d;
    stroke: #ff9800;
}

.node.normal rect {
    fill: var(--bg-tertiary);
    stroke: var(--border);
}

.node.selected rect {
    stroke: var(--accent);
    stroke-width: 3px;
}

.node:hover rect {
    stroke: var(--accent-secondary);
    cursor: pointer;
}

.node text {
    fill: var(--text-primary);
    font-family: 'Consolas', 'Monaco', monospace;
    font-size: 11px;
}

.node .block-title {
    font-weight: bold;
    fill: var(--accent);
}

.node .block-info {
    fill: var(--text-muted);
    font-size: 10px;
}

/* Edge styles */
.edgePath path {
    stroke: var(--text-muted);
    stroke-width: 1.5px;
    fill: none;
}

.edgePath.true-branch path {
    stroke: #4caf50;
    stroke-width: 2px;
}

.edgePath.false-branch path {
    stroke: #f44336;
    stroke-width: 2px;
}

.edgePath.back-edge path {
    stroke: var(--warning);
    stroke-dasharray: 5, 3;
}

.edgeLabel {
    font-family: 'Consolas', 'Monaco', monospace;
    font-size: 10px;
    font-weight: bold;
}

.edgeLabel .label-true {
    fill: #4caf50;
}

.edgeLabel .label-false {
    fill: #f44336;
}

marker#arrowhead path {
    fill: var(--text-muted);
}

/* View mode tabs */
.view-tabs {
    display: flex;
    background: var(--bg-primary);
    border-bottom: 1px solid var(--border);
    padding: 0 16px;
}

.view-tab {
    padding: 8px 16px;
    cursor: pointer;
    border: none;
    background: transparent;
    color: var(--text-muted);
    font-family: inherit;
    font-size: 12px;
    border-bottom: 2px solid transparent;
    text-transform: uppercase;
    letter-spacing: 0.5px;
}

.view-tab:hover {
    color: var(--text-primary);
}

.view-tab.active {
    color: var(--accent-secondary);
    border-bottom-color: var(--accent-secondary);
}

/* Node tooltip */
.graph-tooltip {
    position: absolute;
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 12px;
    max-width: 350px;
    z-index: 1000;
    box-shadow: 0 4px 12px rgba(0,0,0,0.4);
    pointer-events: none;
    font-size: 11px;
}

.graph-tooltip .tooltip-header {
    color: var(--accent);
    font-weight: bold;
    margin-bottom: 8px;
    font-size: 12px;
}

.graph-tooltip .tooltip-inst {
    color: var(--text-primary);
    margin: 2px 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.graph-tooltip .tooltip-meta {
    color: var(--text-muted);
    margin-top: 8px;
    font-size: 10px;
}

/* Data Flow Graph Nodes */
.node.instruction rect {
    fill: var(--bg-tertiary);
    stroke: var(--accent);
}

.node.value rect {
    fill: #2d3a5a;
    stroke: #569cd6;
    rx: 10;
    ry: 10;
}

.node.const rect {
    fill: #3d2d4a;
    stroke: #c586c0;
    rx: 10;
    ry: 10;
}

.node.param rect {
    fill: #2d4a3d;
    stroke: #4ec9b0;
    rx: 10;
    ry: 10;
}

/* Data Flow Edge Types */
.edgePath.def path {
    stroke: #4ec9b0;
    stroke-width: 2px;
}

.edgePath.use path {
    stroke: #569cd6;
    stroke-width: 1.5px;
}

.edgePath.phi path {
    stroke: #c586c0;
    stroke-dasharray: 3, 2;
}

/* Topology Graph Nodes */
.node.kirchoff_law rect {
    fill: #2d5a3d;
    stroke: #4caf50;
}

.node.branch_current rect,
.node.unnamed_current rect,
.node.port_current rect {
    fill: #5a4d2d;
    stroke: #ff9800;
}

.node.implicit rect {
    fill: #3d2d5a;
    stroke: #9c27b0;
}

/* Topology Edge Types */
.edgePath.jacobian_resist path {
    stroke: #4caf50;
    stroke-width: 2px;
}

.edgePath.jacobian_react path {
    stroke: #2196f3;
    stroke-width: 2px;
    stroke-dasharray: 5, 3;
}

.edgePath.both path {
    stroke: #ff9800;
    stroke-width: 2.5px;
}

/* Jacobian Matrix */
.jacobian-matrix {
    display: inline-block;
    background: var(--bg-secondary);
    padding: 16px;
    border-radius: 8px;
}

.jacobian-title {
    font-size: 14px;
    font-weight: bold;
    color: var(--accent);
    margin-bottom: 12px;
}

.jacobian-stats {
    display: flex;
    gap: 24px;
    margin-bottom: 16px;
    color: var(--text-muted);
    font-size: 12px;
}

.jacobian-grid {
    display: grid;
    gap: 1px;
    background: var(--border);
    border: 1px solid var(--border);
    border-radius: 4px;
    overflow: hidden;
}

.jacobian-cell {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 9px;
    cursor: pointer;
    transition: transform 0.1s;
}

.jacobian-cell:hover {
    transform: scale(1.5);
    z-index: 10;
    position: relative;
}

.jacobian-cell.empty {
    background: var(--bg-primary);
}

.jacobian-cell.resist {
    background: #2d5a3d;
}

.jacobian-cell.react {
    background: #2d3d5a;
}

.jacobian-cell.both {
    background: #5a4d2d;
}

.jacobian-cell.diagonal {
    border: 2px solid var(--accent);
}

.jacobian-header {
    background: var(--bg-tertiary);
    color: var(--text-muted);
    font-size: 8px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    padding: 2px;
}

.jacobian-row-header {
    writing-mode: horizontal-tb;
    text-align: right;
    padding-right: 4px;
}

.jacobian-col-header {
    writing-mode: vertical-rl;
    text-orientation: mixed;
    transform: rotate(180deg);
    text-align: left;
    padding-top: 4px;
}

.jacobian-legend {
    display: flex;
    gap: 16px;
    margin-top: 12px;
    font-size: 11px;
}

.jacobian-legend-item {
    display: flex;
    align-items: center;
    gap: 6px;
}

.jacobian-legend-color {
    width: 16px;
    height: 16px;
    border-radius: 2px;
}
"#;

/// JavaScript for interactivity
pub const JS: &str = r#"
// Global state
let moduleData = null;
let currentFunction = 'eval';
let currentView = 'code';
let graphRenderer = null;
let zoomBehavior = null;
let currentTransform = null;

// Initialize the visualization
function init(data) {
    moduleData = data;
    renderSidebar();
    renderTabs();
    selectFunction('eval');
}

function renderSidebar() {
    const sidebar = document.getElementById('sidebar');
    sidebar.innerHTML = `
        <div class="section">
            <div class="section-header" onclick="toggleSection(this)">
                <h3>Module Info</h3>
                <span class="toggle">▼</span>
            </div>
            <div class="section-content">
                <div class="tree-item">
                    <span class="icon">📦</span>
                    <span>${moduleData.module_name}</span>
                </div>
                ${moduleData.ports.map(p => `
                    <div class="tree-item">
                        <span class="icon">🔌</span>
                        <span>${p}</span>
                    </div>
                `).join('')}
            </div>
        </div>
        <div class="section">
            <div class="section-header" onclick="toggleSection(this)">
                <h3>DAE System</h3>
                <span class="toggle">▼</span>
            </div>
            <div class="section-content">
                <div class="tree-item">
                    <span class="icon">📊</span>
                    <span>Unknowns: ${moduleData.dae_system.num_unknowns}</span>
                </div>
                <div class="tree-item">
                    <span class="icon">⚡</span>
                    <span>Resistive: ${moduleData.dae_system.num_resistive}</span>
                </div>
                <div class="tree-item">
                    <span class="icon">🔄</span>
                    <span>Reactive: ${moduleData.dae_system.num_reactive}</span>
                </div>
                <div class="tree-item">
                    <span class="icon">📐</span>
                    <span>Jacobian: ${moduleData.dae_system.jacobian_entries}</span>
                </div>
            </div>
        </div>
        <div class="section">
            <div class="section-header" onclick="toggleSection(this)">
                <h3>Parameters</h3>
                <span class="toggle">▼</span>
            </div>
            <div class="section-content">
                ${moduleData.parameters.slice(0, 20).map(p => `
                    <div class="tree-item">
                        <span class="icon">⚙️</span>
                        <span title="${p.kind}">${p.name}</span>
                    </div>
                `).join('')}
                ${moduleData.parameters.length > 20 ? `
                    <div class="tree-item">
                        <span class="icon">...</span>
                        <span>+${moduleData.parameters.length - 20} more</span>
                    </div>
                ` : ''}
            </div>
        </div>
    `;
}

function renderTabs() {
    const tabs = document.getElementById('tabs');
    const functions = ['eval', 'init', 'model_param_setup'];
    tabs.innerHTML = functions.map(f => {
        const funcData = moduleData[f];
        if (!funcData) return '';
        return `<button class="tab ${f === currentFunction ? 'active' : ''}"
                        onclick="selectFunction('${f}')">${f}</button>`;
    }).join('');
}

function selectFunction(name) {
    currentFunction = name;
    document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
    document.querySelector(`.tab[onclick="selectFunction('${name}')"]`)?.classList.add('active');
    renderFunctionView();
}

function renderFunctionView() {
    const func = moduleData[currentFunction];
    if (!func) {
        document.getElementById('panel').innerHTML = '<p>Function not available</p>';
        return;
    }

    const panel = document.getElementById('panel');
    panel.innerHTML = `
        <div class="view-tabs">
            <button class="view-tab ${currentView === 'code' ? 'active' : ''}"
                    onclick="switchView('code')">Code</button>
            <button class="view-tab ${currentView === 'topology' ? 'active' : ''}"
                    onclick="switchView('topology')">Topology</button>
            <button class="view-tab ${currentView === 'jacobian' ? 'active' : ''}"
                    onclick="switchView('jacobian')">Jacobian</button>
            <button class="view-tab ${currentView === 'summary' ? 'active' : ''}"
                    onclick="switchView('summary')">Summary</button>
        </div>
        <div id="code-view" style="display: ${currentView === 'code' ? 'block' : 'none'}; height: calc(100% - 40px); overflow: auto;">
            <div class="stats">
                <div class="stat">
                    <div class="stat-value">${func.num_blocks}</div>
                    <div class="stat-label">Blocks</div>
                </div>
                <div class="stat">
                    <div class="stat-value">${func.num_instructions}</div>
                    <div class="stat-label">Instructions</div>
                </div>
                <div class="stat">
                    <div class="stat-value">${func.num_values}</div>
                    <div class="stat-label">Values</div>
                </div>
            </div>
            <div id="blocks-container">
                ${func.blocks.map(block => renderBlock(block)).join('')}
            </div>
        </div>
        <div id="topology-view" class="graph-container" style="display: ${currentView === 'topology' ? 'block' : 'none'}; height: calc(100% - 40px);">
            <div class="graph-controls">
                <button class="graph-btn" onclick="zoomInTopology()">+</button>
                <button class="graph-btn" onclick="zoomOutTopology()">-</button>
                <button class="graph-btn" onclick="resetZoomTopology()">Reset</button>
                <button class="graph-btn" onclick="fitTopologyGraph()">Fit</button>
            </div>
            <div style="position: absolute; top: 10px; left: 16px; max-width: 400px; background: var(--bg-tertiary); border: 1px solid var(--border); border-radius: 4px; padding: 8px 12px; font-size: 11px; z-index: 100;">
                <strong style="color: var(--accent);">Jacobian Dependency Graph</strong><br>
                <span style="color: var(--text-muted);">
                    Nodes are unknowns (voltages/currents). Arrows show Jacobian dependencies:
                    A→B means equation for B depends on variable A (∂f<sub>B</sub>/∂x<sub>A</sub> ≠ 0).
                    Colors: <span style="color: #4caf50;">■</span> resistive,
                    <span style="color: #2196f3;">■</span> reactive,
                    <span style="color: #ff9800;">■</span> both.
                </span>
            </div>
            <div id="topology-graph"></div>
            <div id="topology-tooltip" class="graph-tooltip" style="display: none;"></div>
        </div>
        <div id="jacobian-view" style="display: ${currentView === 'jacobian' ? 'block' : 'none'}; height: calc(100% - 40px); overflow: auto; padding: 16px;">
            <div id="jacobian-container"></div>
        </div>
        <div id="summary-view" style="display: ${currentView === 'summary' ? 'block' : 'none'}; height: calc(100% - 40px); overflow: auto; padding: 16px;">
            <pre>${window.TEXT_SUMMARY || 'Summary not available'}</pre>
        </div>
    `;

    if (currentView === 'topology') {
        setTimeout(() => renderTopologyGraph(), 50);
    } else if (currentView === 'jacobian') {
        setTimeout(() => renderJacobianMatrix(), 50);
    }
}

function switchView(view) {
    currentView = view;
    renderFunctionView();
}

function renderBlock(block) {
    return `
        <div class="block" id="${block.id}">
            <div class="block-header" onclick="toggleBlock('${block.id}')">
                <span class="block-name">${block.id}</span>
                <span class="block-meta">
                    ${block.instructions.length} insts |
                    pred: ${block.predecessors.join(', ') || 'none'} |
                    succ: ${block.successors.join(', ') || 'none'}
                </span>
            </div>
            <div class="block-content" id="${block.id}-content">
                ${block.instructions.map(inst => renderInstruction(inst)).join('')}
            </div>
        </div>
    `;
}

function renderInstruction(inst) {
    const categoryClass = inst.opcode_category || '';
    const result = inst.results.length > 0 ? inst.results.join(', ') + ' =' : '';
    const args = inst.arguments.join(', ');

    return `
        <div class="instruction" data-inst="${inst.id}">
            <span class="inst-result">${result}</span>
            <span class="inst-opcode ${categoryClass}">${inst.opcode}</span>
            <span class="inst-args">${args}</span>
        </div>
    `;
}

function toggleSection(header) {
    const content = header.nextElementSibling;
    const toggle = header.querySelector('.toggle');
    content.classList.toggle('collapsed');
    toggle.textContent = content.classList.contains('collapsed') ? '▶' : '▼';
}

function toggleBlock(blockId) {
    const content = document.getElementById(`${blockId}-content`);
    content.style.display = content.style.display === 'none' ? 'block' : 'none';
}

// Search functionality
function setupSearch() {
    const searchInput = document.getElementById('search-input');
    if (searchInput) {
        searchInput.addEventListener('input', (e) => {
            const query = e.target.value.toLowerCase();
            filterInstructions(query);
        });
    }
}

function filterInstructions(query) {
    if (!query) {
        document.querySelectorAll('.instruction').forEach(el => el.style.display = 'flex');
        document.querySelectorAll('.block').forEach(el => el.style.display = 'block');
        return;
    }

    document.querySelectorAll('.block').forEach(block => {
        let hasMatch = false;
        block.querySelectorAll('.instruction').forEach(inst => {
            const text = inst.textContent.toLowerCase();
            if (text.includes(query)) {
                inst.style.display = 'flex';
                hasMatch = true;
            } else {
                inst.style.display = 'none';
            }
        });
        block.style.display = hasMatch ? 'block' : 'none';
    });
}

// ==================== TOPOLOGY GRAPH ====================
let topologyZoom = null;

function renderTopologyGraph() {
    const dae = moduleData.dae_system;
    if (!dae || !dae.topology) return;

    const container = document.getElementById('topology-graph');
    if (!container) return;

    const width = container.clientWidth || 800;
    const height = container.clientHeight || 600;

    container.innerHTML = '';

    const svg = d3.select(container)
        .append('svg')
        .attr('width', width)
        .attr('height', height);

    svg.append('defs').append('marker')
        .attr('id', 'topology-arrow')
        .attr('viewBox', '0 -5 10 10')
        .attr('refX', 8)
        .attr('refY', 0)
        .attr('markerWidth', 6)
        .attr('markerHeight', 6)
        .attr('orient', 'auto')
        .append('path')
        .attr('d', 'M0,-5L10,0L0,5')
        .attr('fill', '#808080');

    const g = svg.append('g');

    const dagreGraph = new dagre.graphlib.Graph()
        .setGraph({ rankdir: 'TB', nodesep: 60, ranksep: 80, marginx: 40, marginy: 40 })
        .setDefaultEdgeLabel(() => ({}));

    // Add nodes
    dae.topology.nodes.forEach(node => {
        dagreGraph.setNode(node.id.toString(), {
            label: node.name,
            width: 100,
            height: 50,
            nodeType: node.kind
        });
    });

    // Add edges
    dae.topology.edges.forEach(edge => {
        dagreGraph.setEdge(edge.from.toString(), edge.to.toString(), { edgeType: edge.edge_type });
    });

    dagre.layout(dagreGraph);

    // Draw edges
    g.selectAll('.edgePath')
        .data(dagreGraph.edges())
        .enter()
        .append('g')
        .attr('class', d => `edgePath ${dagreGraph.edge(d).edgeType}`)
        .append('path')
        .attr('d', d => {
            const edge = dagreGraph.edge(d);
            let path = `M${edge.points[0].x},${edge.points[0].y}`;
            for (let i = 1; i < edge.points.length; i++) {
                path += `L${edge.points[i].x},${edge.points[i].y}`;
            }
            return path;
        })
        .attr('marker-end', 'url(#topology-arrow)');

    // Draw nodes
    const nodes = g.selectAll('.node')
        .data(dagreGraph.nodes())
        .enter()
        .append('g')
        .attr('class', d => `node ${dagreGraph.node(d).nodeType}`)
        .attr('transform', d => {
            const node = dagreGraph.node(d);
            return `translate(${node.x - node.width/2}, ${node.y - node.height/2})`;
        });

    nodes.append('rect')
        .attr('width', d => dagreGraph.node(d).width)
        .attr('height', d => dagreGraph.node(d).height);

    nodes.append('text')
        .attr('x', d => dagreGraph.node(d).width / 2)
        .attr('y', d => dagreGraph.node(d).height / 2 + 4)
        .attr('text-anchor', 'middle')
        .text(d => {
            const label = dagreGraph.node(d).label;
            return label.length > 12 ? label.substring(0, 10) + '..' : label;
        });

    topologyZoom = d3.zoom()
        .scaleExtent([0.1, 4])
        .on('zoom', (event) => g.attr('transform', event.transform));

    svg.call(topologyZoom);
    fitTopologyGraph();
}

function zoomInTopology() {
    d3.select('#topology-graph svg').transition().duration(300).call(topologyZoom.scaleBy, 1.3);
}
function zoomOutTopology() {
    d3.select('#topology-graph svg').transition().duration(300).call(topologyZoom.scaleBy, 0.7);
}
function resetZoomTopology() {
    d3.select('#topology-graph svg').transition().duration(300).call(topologyZoom.transform, d3.zoomIdentity);
}
function fitTopologyGraph() {
    const svg = d3.select('#topology-graph svg');
    const g = svg.select('g');
    if (!svg.node() || !g.node()) return;
    const bounds = g.node().getBBox();
    const width = svg.node().clientWidth;
    const height = svg.node().clientHeight;
    if (bounds.width === 0) return;
    const scale = Math.min(0.9 * width / bounds.width, 0.9 * height / bounds.height, 1.5);
    const tx = (width - bounds.width * scale) / 2 - bounds.x * scale;
    const ty = (height - bounds.height * scale) / 2 - bounds.y * scale;
    svg.transition().duration(500).call(topologyZoom.transform, d3.zoomIdentity.translate(tx, ty).scale(scale));
}

// ==================== JACOBIAN MATRIX ====================
function renderJacobianMatrix() {
    const dae = moduleData.dae_system;
    if (!dae) return;

    const container = document.getElementById('jacobian-container');
    if (!container) return;

    const n = dae.num_unknowns;
    const jacobian = dae.jacobian || [];
    const unknowns = dae.unknowns || [];

    // Build sparse matrix representation
    const matrix = {};
    jacobian.forEach(entry => {
        const key = `${entry.row},${entry.col}`;
        matrix[key] = entry;
    });

    // Count entries
    let resistCount = 0, reactCount = 0, bothCount = 0;
    jacobian.forEach(e => {
        if (e.has_resist && e.has_react) bothCount++;
        else if (e.has_resist) resistCount++;
        else if (e.has_react) reactCount++;
    });

    let html = `
        <div class="jacobian-matrix">
            <div class="jacobian-title">Jacobian Matrix (${n} x ${n})</div>
            <div class="jacobian-stats">
                <span>Total entries: ${jacobian.length}</span>
                <span>Resistive: ${resistCount + bothCount}</span>
                <span>Reactive: ${reactCount + bothCount}</span>
                <span>Sparsity: ${((1 - jacobian.length / (n * n)) * 100).toFixed(1)}%</span>
            </div>
    `;

    if (n <= 30) {
        // Small matrix - show full grid
        html += `<div class="jacobian-grid" style="grid-template-columns: 80px repeat(${n}, 24px);">`;

        // Header row
        html += `<div class="jacobian-header"></div>`;
        for (let j = 0; j < n; j++) {
            const name = unknowns[j]?.name || j.toString();
            html += `<div class="jacobian-header jacobian-col-header" title="${name}">${name}</div>`;
        }

        // Data rows
        for (let i = 0; i < n; i++) {
            const rowName = unknowns[i]?.name || i.toString();
            html += `<div class="jacobian-header jacobian-row-header" title="${rowName}">${rowName}</div>`;

            for (let j = 0; j < n; j++) {
                const entry = matrix[`${i},${j}`];
                let cellClass = 'jacobian-cell empty';
                let title = `(${i}, ${j})`;

                if (entry) {
                    if (entry.has_resist && entry.has_react) {
                        cellClass = 'jacobian-cell both';
                        title = `${entry.row_name} / ${entry.col_name}: resist + react`;
                    } else if (entry.has_resist) {
                        cellClass = 'jacobian-cell resist';
                        title = `${entry.row_name} / ${entry.col_name}: resist`;
                    } else if (entry.has_react) {
                        cellClass = 'jacobian-cell react';
                        title = `${entry.row_name} / ${entry.col_name}: react`;
                    }
                }
                if (i === j) cellClass += ' diagonal';

                html += `<div class="${cellClass}" title="${title}"></div>`;
            }
        }
        html += '</div>';
    } else {
        // Large matrix - show summary
        html += `<p style="color: var(--text-muted);">Matrix too large for grid view (${n}x${n}). Showing entry list:</p>`;
        html += `<div style="max-height: 400px; overflow-y: auto;">`;
        jacobian.slice(0, 100).forEach(e => {
            const type = e.has_resist && e.has_react ? 'both' :
                         e.has_resist ? 'resist' : 'react';
            html += `<div style="padding: 4px; border-bottom: 1px solid var(--border);">
                <span style="color: var(--accent);">[${e.row}, ${e.col}]</span>
                ${e.row_name} / ${e.col_name}:
                <span style="color: ${type === 'resist' ? '#4caf50' : type === 'react' ? '#2196f3' : '#ff9800'};">${type}</span>
            </div>`;
        });
        if (jacobian.length > 100) {
            html += `<div style="padding: 8px; color: var(--text-muted);">... and ${jacobian.length - 100} more entries</div>`;
        }
        html += '</div>';
    }

    html += `
        <div class="jacobian-legend">
            <div class="jacobian-legend-item">
                <div class="jacobian-legend-color" style="background: #2d5a3d;"></div>
                <span>Resistive (I)</span>
            </div>
            <div class="jacobian-legend-item">
                <div class="jacobian-legend-color" style="background: #2d3d5a;"></div>
                <span>Reactive (Q)</span>
            </div>
            <div class="jacobian-legend-item">
                <div class="jacobian-legend-color" style="background: #5a4d2d;"></div>
                <span>Both</span>
            </div>
        </div>
    </div>`;

    container.innerHTML = html;
}

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    if (window.MODULE_DATA) {
        init(window.MODULE_DATA);
        setupSearch();
    }
});
"#;
