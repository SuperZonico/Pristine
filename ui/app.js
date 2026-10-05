/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         ui/app.js
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Frontend application logic, Tauri IPC bindings, theme sync,
 *               and reactive telemetry rendering.
 * ============================================================================
 */

(() => {
  // State Store
  const state = {
    catalog: [],
    audit: null,
    sessions: [],
    selectedTweakIds: new Set(),
    themeMode: 'dark', // 'dark', 'light', 'auto'
  };

  // Safe Tauri Invoke Wrapper with Browser Fallback
  async function invokeNative(cmd, args = {}) {
    const invoker =
      (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) ||
      (window.__TAURI__ && window.__TAURI__.invoke) ||
      (window.__TAURI_INTERNALS__ && window.__TAURI_INTERNALS__.invoke);

    if (invoker) {
      try {
        return await invoker(cmd, args);
      } catch (err) {
        console.error(`Tauri command '${cmd}' failed:`, err);
        throw err;
      }
    }

    // Fallback Mock Data for Browser / Dev Preview
    if (cmd === 'get_catalog') {
      return [
        {
          id: 'diagtrack_utc',
          title: 'Connected User Experiences & Telemetry (DiagTrack)',
          description: 'Deshabilita el servicio DiagTrack de recopilación y telemetría diagnóstica hacia Microsoft.',
          category: 'telemetry',
          risk: 'safe',
          impact_details: 'Detiene el envío periódico de registros de diagnóstico. No afecta Windows Update ni la Store.',
          default_recommended: true
        },
        {
          id: 'advertising_id',
          title: 'Identificador de Publicidad de Windows',
          description: 'Impide que las aplicaciones rastreen la actividad del usuario entre programas.',
          category: 'privacy',
          risk: 'safe',
          impact_details: 'Elimina anuncios personalizados en aplicaciones de la Tienda de Windows.',
          default_recommended: true
        },
        {
          id: 'bing_start_search',
          title: 'Búsqueda Web con Bing en Menú Inicio',
          description: 'Evita que cada pulsación en el menú inicio envíe peticiones HTTP a servidores de Bing.',
          category: 'privacy',
          risk: 'safe',
          impact_details: 'La búsqueda en el menú inicio se vuelve instantánea y limitada a archivos locales.',
          default_recommended: true
        },
        {
          id: 'windows_recall_ai',
          title: 'Windows Recall y Capturas Continuas de IA',
          description: 'Aplica directivas oficiales para deshabilitar las capturas de pantalla continuas y OCR local.',
          category: 'privacy',
          risk: 'safe',
          impact_details: 'Protege contra infostealers y mitiga el registro constante de actividad de pantalla.',
          default_recommended: true
        },
        {
          id: 'copilot_ai',
          title: 'Integración de Windows Copilot',
          description: 'Desactiva el panel lateral de Copilot y sus directivas de nube asociadas.',
          category: 'privacy',
          risk: 'moderate',
          impact_details: 'Oculta el acceso directo a Copilot en la barra de tareas. Se puede revertir en 1 clic.',
          default_recommended: false
        },
        {
          id: 'delivery_optimization_p2p',
          title: 'Optimización de Entrega P2P hacia Internet',
          description: 'Restringe la distribución P2P de actualizaciones exclusivamente a la red local.',
          category: 'system_cleanup',
          risk: 'safe',
          impact_details: 'Ahorra ancho de banda de subida a Internet sin alterar las descargas oficiales.',
          default_recommended: true
        },
        {
          id: 'multimedia_network_throttling',
          title: 'Mitigación de Throttling Multimedia de Red',
          description: 'Desactiva el estrangulamiento de paquetes de red de fondo durante reproducción de audio.',
          category: 'latency',
          risk: 'safe',
          impact_details: 'Reduce el jitter y latencia en juegos competitivos y software de producción de audio.',
          default_recommended: true
        }
      ];
    }

    if (cmd === 'audit_system') {
      return {
        timestamp_utc: Math.floor(Date.now() / 1000),
        total_analyzed: 7,
        active_count: 5,
        inactive_count: 2,
        privacy_score_percent: 71,
        statuses: [
          { tweak_id: 'diagtrack_utc', state: 'active', last_checked_epoch: Date.now() },
          { tweak_id: 'advertising_id', state: 'active', last_checked_epoch: Date.now() },
          { tweak_id: 'bing_start_search', state: 'active', last_checked_epoch: Date.now() },
          { tweak_id: 'windows_recall_ai', state: 'active', last_checked_epoch: Date.now() },
          { tweak_id: 'copilot_ai', state: 'inactive', last_checked_epoch: Date.now() },
          { tweak_id: 'delivery_optimization_p2p', state: 'active', last_checked_epoch: Date.now() },
          { tweak_id: 'multimedia_network_throttling', state: 'inactive', last_checked_epoch: Date.now() }
        ]
      };
    }

    if (cmd === 'get_hardware_metrics') {
      return {
        ram_total_mb: 32768,
        ram_used_mb: 7420,
        ram_percent: 23,
        cpu_percent: Math.floor(Math.random() * 8) + 2,
        timestamp_utc: Math.floor(Date.now() / 1000)
      };
    }

    if (cmd === 'clean_safe_temporary_files') {
      return {
        bytes_freed: 1428570000,
        files_deleted: 134,
        errors_encountered: 0
      };
    }

    if (cmd === 'apply_tweaks') {
      return {
        session_id: 'sess_' + Date.now(),
        created_at_utc: Math.floor(Date.now() / 1000),
        description: args.description || 'Optimización manual',
        registry_rollbacks: [],
        service_rollbacks: [],
        task_rollbacks: [],
        integrity_hash: 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855'
      };
    }

    return null;
  }

  // DOM Elements
  const elScore = document.getElementById('stat-score');
  const elScoreLabel = document.getElementById('stat-score-label');
  const elMeterCircle = document.getElementById('meter-circle');
  const elActiveCount = document.getElementById('stat-active-count');
  const elTotalCount = document.getElementById('stat-total-count');

  const elCpuVal = document.getElementById('val-cpu');
  const elCpuFill = document.getElementById('fill-cpu');
  const elRamVal = document.getElementById('val-ram');
  const elRamFill = document.getElementById('fill-ram');

  const elPrivacyList = document.getElementById('privacy-tweaks-list');
  const elServicesList = document.getElementById('services-list');
  const elRollbackList = document.getElementById('rollback-list');

  // Navigation Logic
  function setupNavigation() {
    document.querySelectorAll('.nav-item').forEach(item => {
      item.addEventListener('click', () => {
        const target = item.getAttribute('data-target');
        document.querySelectorAll('.nav-item').forEach(n => n.classList.remove('active'));
        document.querySelectorAll('.view-section').forEach(v => v.classList.remove('active'));

        item.classList.add('active');
        const view = document.getElementById(target);
        if (view) view.classList.add('active');
      });
    });
  }

  // Theme Logic
  function applyTheme(theme) {
    state.themeMode = theme;
    document.querySelectorAll('.theme-btn').forEach(b => b.classList.remove('active'));

    if (theme === 'auto') {
      document.getElementById('btn-theme-auto').classList.add('active');
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      document.documentElement.setAttribute('data-theme', prefersDark ? 'dark' : 'light');
    } else {
      document.getElementById(`btn-theme-${theme}`).classList.add('active');
      document.documentElement.setAttribute('data-theme', theme);
    }
  }

  function setupThemeHandlers() {
    document.getElementById('btn-theme-light').addEventListener('click', () => applyTheme('light'));
    document.getElementById('btn-theme-dark').addEventListener('click', () => applyTheme('dark'));
    document.getElementById('btn-theme-auto').addEventListener('click', () => applyTheme('auto'));

    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', e => {
      if (state.themeMode === 'auto') {
        document.documentElement.setAttribute('data-theme', e.matches ? 'dark' : 'light');
      }
    });
  }

  // Update Privacy Score Meter
  function updatePrivacyMeter(percent) {
    elScore.textContent = `${percent}%`;
    const circumference = 440; // 2 * PI * 70
    const offset = circumference - (circumference * percent) / 100;
    elMeterCircle.style.strokeDashoffset = offset;

    if (percent >= 80) {
      elMeterCircle.style.stroke = 'var(--accent-emerald)';
      elScoreLabel.textContent = 'Protegido';
    } else if (percent >= 50) {
      elMeterCircle.style.stroke = 'var(--accent-amber)';
      elScoreLabel.textContent = 'Moderado';
    } else {
      elMeterCircle.style.stroke = 'var(--accent-rose)';
      elScoreLabel.textContent = 'Expuesto';
    }
  }

  // Live Metrics Loop
  async function pollHardwareMetrics() {
    if (document.hidden) return; // Pause polling when minimized to consume 0% CPU

    try {
      const metrics = await invokeNative('get_hardware_metrics');
      if (metrics) {
        elCpuVal.textContent = `${metrics.cpu_percent}%`;
        elCpuFill.style.width = `${metrics.cpu_percent}%`;

        elRamVal.textContent = `${metrics.ram_percent}%`;
        elRamFill.style.width = `${metrics.ram_percent}%`;
      }
    } catch (_) {}
  }

  // Render Tweak Cards
  function renderTweakCard(tweak, status) {
    const card = document.createElement('div');
    card.className = 'tweak-card';
    card.id = `card-${tweak.id}`;

    const isActive = status && status.state === 'active';
    if (isActive) card.classList.add('active-card');

    const riskClass = tweak.risk === 'safe' ? 'risk-safe' : tweak.risk === 'moderate' ? 'risk-moderate' : 'risk-critical';
    const riskLabel = tweak.risk === 'safe' ? 'Seguro' : tweak.risk === 'moderate' ? 'Moderado' : 'Avanzado';

    card.innerHTML = `
      <div class="tweak-main-row">
        <div class="tweak-info">
          <div class="tweak-title-row">
            <span class="tweak-title">${tweak.title}</span>
            <span class="risk-badge ${riskClass}">${riskLabel}</span>
          </div>
          <p class="tweak-desc">${tweak.description}</p>
        </div>
        <label class="switch">
          <input type="checkbox" id="toggle-${tweak.id}" ${isActive ? 'checked' : ''}>
          <span class="slider"></span>
        </label>
      </div>

      <button class="tweak-drawer-toggle" data-drawer="drawer-${tweak.id}">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>
        <span>Detalles técnicos de impacto</span>
      </button>

      <div class="tweak-drawer" id="drawer-${tweak.id}">
        <div class="drawer-field"><strong>Impacto en Sistema:</strong> ${tweak.impact_details}</div>
        <div class="drawer-field"><strong>Categoría:</strong> ${tweak.category}</div>
        <div class="drawer-field"><strong>Reversibilidad:</strong> 100% Garantizada vía diario transaccional</div>
      </div>
    `;

    // Drawer toggle handler
    const btnDrawer = card.querySelector('.tweak-drawer-toggle');
    const drawer = card.querySelector('.tweak-drawer');
    btnDrawer.addEventListener('click', () => {
      drawer.classList.toggle('open');
    });

    // Checkbox toggle handler
    const checkbox = card.querySelector(`#toggle-${tweak.id}`);
    checkbox.addEventListener('change', () => {
      if (checkbox.checked) {
        state.selectedTweakIds.add(tweak.id);
        card.classList.add('active-card');
      } else {
        state.selectedTweakIds.delete(tweak.id);
        card.classList.remove('active-card');
      }
    });

    return card;
  }

  // Load and Render Views
  async function loadData() {
    try {
      state.catalog = await invokeNative('get_catalog');
      if (state.catalog && state.catalog.length > 0) {
        renderCatalog();
      }

      state.audit = await invokeNative('audit_system');
      if (state.audit) {
        updatePrivacyMeter(state.audit.privacy_score_percent);
        elActiveCount.textContent = state.audit.active_count;
        elTotalCount.textContent = state.audit.total_analyzed;
        renderCatalog();
      }
    } catch (err) {
      console.error('Error loading catalog and audit:', err);
    }
  }

  function renderCatalog() {
    elPrivacyList.innerHTML = '';
    elServicesList.innerHTML = '';

    state.catalog.forEach(tweak => {
      const status = state.audit ? state.audit.statuses.find(s => s.tweak_id === tweak.id) : null;
      if (status && status.state === 'active') {
        state.selectedTweakIds.add(tweak.id);
      }

      const card = renderTweakCard(tweak, status);
      elPrivacyList.appendChild(card);

      if (tweak.category === 'telemetry') {
        const srvCard = renderTweakCard(tweak, status);
        elServicesList.appendChild(srvCard);
      }
    });
  }

  // Setup Action Handlers
  function setupActions() {
    // Quick Scan
    document.getElementById('btn-quick-scan').addEventListener('click', async () => {
      const btn = document.getElementById('btn-quick-scan');
      btn.disabled = true;
      btn.querySelector('span').textContent = 'Escaneando...';

      await loadData();

      btn.disabled = false;
      btn.querySelector('span').textContent = 'Escanear Sistema';
    });

    // Apply Recommended
    document.getElementById('btn-apply-recommended').addEventListener('click', () => {
      state.catalog.forEach(tweak => {
        if (tweak.default_recommended) {
          state.selectedTweakIds.add(tweak.id);
          const toggle = document.getElementById(`toggle-${tweak.id}`);
          if (toggle) toggle.checked = true;
          const card = document.getElementById(`card-${tweak.id}`);
          if (card) card.classList.add('active-card');
        }
      });
    });

    // Save Privacy Changes
    document.getElementById('btn-save-privacy').addEventListener('click', async () => {
      const btn = document.getElementById('btn-save-privacy');
      btn.disabled = true;
      btn.querySelector('span').textContent = 'Aplicando...';

      try {
        const session = await invokeNative('apply_tweaks', {
          tweakIds: Array.from(state.selectedTweakIds),
          description: 'Ajuste manual de directivas de privacidad'
        });

        if (session) {
          state.sessions.unshift(session);
          renderRollbackHistory();
        }

        await loadData();
      } catch (err) {
        alert('Error al aplicar cambios: ' + err);
      } finally {
        btn.disabled = false;
        btn.querySelector('span').textContent = 'Aplicar Cambios';
      }
    });

    // Safe Cleaner
    const btnClean = document.getElementById('btn-run-cleaner');
    const elCleanStatus = document.getElementById('clean-status');
    const btnQuickClean = document.getElementById('btn-quick-clean');

    async function runCleaning() {
      btnClean.disabled = true;
      elCleanStatus.textContent = 'Analizando y purgando archivos...';

      try {
        const res = await invokeNative('clean_safe_temporary_files');
        if (res) {
          const mbFreed = (res.bytes_freed / (1024 * 1024)).toFixed(1);
          elCleanStatus.textContent = `¡Liberados ${mbFreed} MB (${res.files_deleted} archivos seguros eliminados)!`;
        }
      } catch (err) {
        elCleanStatus.textContent = 'Error al ejecutar limpieza: ' + err;
      } finally {
        btnClean.disabled = false;
      }
    }

    btnClean.addEventListener('click', runCleaning);
    btnQuickClean.addEventListener('click', runCleaning);
  }

  // Render Rollback Timeline
  function renderRollbackHistory() {
    if (state.sessions.length === 0) return;

    elRollbackList.innerHTML = '';
    state.sessions.forEach(session => {
      const item = document.createElement('div');
      item.className = 'timeline-item';

      const dateStr = new Date(session.created_at_utc * 1000).toLocaleString();
      const shortHash = session.integrity_hash ? session.integrity_hash.substring(0, 16) + '...' : 'OK';

      item.innerHTML = `
        <div class="timeline-info">
          <div class="timeline-id">${session.description}</div>
          <div class="timeline-meta">${dateStr} &bull; Hash SHA-256: <span class="hash-pill">${shortHash}</span></div>
        </div>
        <button class="btn btn-secondary btn-revert" data-id="${session.session_id}">
          <svg viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>
          <span>Revertir Sesión</span>
        </button>
      `;

      item.querySelector('.btn-revert').addEventListener('click', async () => {
        if (confirm(`¿Revertir la sesión '${session.description}' al estado exacto previo?`)) {
          try {
            await invokeNative('revert_transaction', { session });
            alert('Sesión revertida exitosamente.');
            await loadData();
          } catch (err) {
            alert('Error al revertir sesión: ' + err);
          }
        }
      });

      elRollbackList.appendChild(item);
    });
  }

  // App Initialization
  function initApp() {
    console.log('[PRISTINE] Initializing UI application...');
    setupNavigation();
    setupThemeHandlers();
    setupActions();
    loadData();

    // Start live metrics loop
    pollHardwareMetrics();
    setInterval(pollHardwareMetrics, 1500);

    invokeNative('frontend_log', { msg: 'DOM and event handlers initialized successfully' }).catch(() => {});
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initApp);
  } else {
    initApp();
  }
})();
