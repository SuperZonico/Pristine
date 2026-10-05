/*
 * ============================================================================
 * Project:      Pristine — Privacy & Performance Suite
 * File:         ui/app.js
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Frontend application logic, Tauri IPC bindings, reactive state,
 *               category filtering, VSS snapshot trigger, and rollback manager.
 * ============================================================================
 */

(() => {
  // State Store
  const state = {
    catalog: [],
    audit: null,
    sessions: [],
    selectedTweakIds: new Set(),
    selectedCategory: 'all',
    themeMode: 'dark', // 'dark', 'light', 'auto'
    isElevated: false,
    apps: [],
    selectedAppFilter: 'all',
    appSearchQuery: '',
    hostsShieldActive: false,
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

    if (cmd === 'clean_winsxs_component_store') {
      return 'Limpieza de almacén de componentes DISM completada con éxito. Código de salida: 0.';
    }

    if (cmd === 'create_system_restore_point') {
      return 'Punto de restauración VSS "Pristine Safe Optimization" creado exitosamente.';
    }

    if (cmd === 'get_transaction_history') {
      return [];
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

    if (cmd === 'check_elevation') {
      return true;
    }

    if (cmd === 'request_elevation') {
      return true;
    }

    if (cmd === 'get_installed_apps') {
      return [
        {
          id: 'Microsoft.BingNews_8wekyb3d8bbwe',
          name: 'Noticias de Microsoft',
          publisher: 'Microsoft Corporation',
          version: '4.54.12002.0',
          install_date: 'UWP Package',
          estimated_size_mb: 48,
          uninstall_cmd: '',
          quiet_uninstall_cmd: null,
          is_uwp: true,
          is_system_component: false,
          category: 'bloatware'
        },
        {
          id: 'Microsoft.MicrosoftSolitaireCollection_8wekyb3d8bbwe',
          name: 'Colección de Solitario',
          publisher: 'Microsoft Studios',
          version: '4.17.1120.0',
          install_date: 'UWP Package',
          estimated_size_mb: 64,
          uninstall_cmd: '',
          quiet_uninstall_cmd: null,
          is_uwp: true,
          is_system_component: false,
          category: 'bloatware'
        },
        {
          id: 'Google Chrome',
          name: 'Google Chrome',
          publisher: 'Google LLC',
          version: '129.0.6668.90',
          install_date: '20240920',
          estimated_size_mb: 285,
          uninstall_cmd: 'MsiExec.exe /X{...}',
          quiet_uninstall_cmd: null,
          is_uwp: false,
          is_system_component: false,
          category: 'user'
        },
        {
          id: 'Microsoft Visual C++ 2015-2022 Redistributable (x64)',
          name: 'Microsoft Visual C++ 2015-2022 Redistributable (x64)',
          publisher: 'Microsoft Corporation',
          version: '14.40.33810',
          install_date: '20240815',
          estimated_size_mb: 32,
          uninstall_cmd: '',
          quiet_uninstall_cmd: null,
          is_uwp: false,
          is_system_component: true,
          category: 'system'
        }
      ];
    }

    if (cmd === 'flush_dns') {
      return { success: true, message: 'Caché del resolver DNS purgada exitosamente.' };
    }

    if (cmd === 'get_hosts_shield_status') {
      return false;
    }

    if (cmd === 'toggle_hosts_shield') {
      return { success: true, message: args.enable ? 'Escudo hosts activado (0.0.0.0 sinkhole).' : 'Escudo hosts desactivado.' };
    }

    if (cmd === 'restart_windows_explorer') {
      return { success: true, message: 'Explorador de Windows reiniciado exitosamente.' };
    }

    if (cmd === 'uninstall_app') {
      return { success: true, message: 'Aplicación desinstalada exitosamente.' };
    }

    return null;
  }

  // ============================================================================
  // Native Obsidian-Fluent Modal System (Zero native alert/confirm popups)
  // ============================================================================
  function showModal({
    title,
    message,
    type = 'info', // 'info', 'warning', 'error'
    confirmText = 'Aceptar',
    cancelText = null,
    onConfirm = null,
    onCancel = null
  }) {
    const overlay = document.getElementById('modal-overlay');
    const elTitle = document.getElementById('modal-title');
    const elMessage = document.getElementById('modal-message');
    const elIcon = document.getElementById('modal-icon');
    const btnConfirm = document.getElementById('modal-btn-confirm');
    const btnCancel = document.getElementById('modal-btn-cancel');

    if (!overlay || !elTitle || !elMessage) {
      if (cancelText) {
        if (confirm(message.replace(/<[^>]*>/g, '')) && onConfirm) onConfirm();
        else if (onCancel) onCancel();
      } else {
        alert(message.replace(/<[^>]*>/g, ''));
        if (onConfirm) onConfirm();
      }
      return;
    }

    elTitle.textContent = title;
    elMessage.innerHTML = message;

    elIcon.className = `modal-icon ${type}`;
    if (type === 'warning') {
      elIcon.innerHTML = `<svg viewBox="0 0 24 24"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>`;
    } else if (type === 'error') {
      elIcon.innerHTML = `<svg viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>`;
    } else {
      elIcon.innerHTML = `<svg viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"/><polyline points="9 12 11 14 15 10"/></svg>`;
    }

    btnConfirm.textContent = confirmText;
    btnConfirm.onclick = () => {
      overlay.style.display = 'none';
      if (onConfirm) onConfirm();
    };

    if (cancelText) {
      btnCancel.style.display = 'inline-flex';
      btnCancel.textContent = cancelText;
      btnCancel.onclick = () => {
        overlay.style.display = 'none';
        if (onCancel) onCancel();
      };
    } else {
      btnCancel.style.display = 'none';
      btnCancel.onclick = null;
    }

    overlay.style.display = 'flex';
  }

  // ============================================================================
  // Privilege Elevation Management & Dynamic UAC Detection
  // ============================================================================
  async function updatePrivilegeState() {
    try {
      const isElevated = await invokeNative('check_elevation');
      state.isElevated = !!isElevated;

      const badge = document.getElementById('privilege-badge');
      const badgeText = document.getElementById('privilege-text');
      const banner = document.getElementById('elevation-banner');

      if (badge && badgeText) {
        if (state.isElevated) {
          badge.className = 'privilege-badge elevated';
          badge.title = 'Pristine se ejecuta con privilegios de Administrador';
          badgeText.textContent = 'Administrador';
          if (banner) banner.style.display = 'none';
        } else {
          badge.className = 'privilege-badge unelevated';
          badge.title = 'Haz clic para reiniciar Pristine con privilegios de Administrador';
          badgeText.textContent = 'Modo Estándar (Elevar)';
          if (banner) banner.style.display = 'flex';
        }
      }
    } catch (e) {
      console.warn('Could not check elevation status:', e);
    }
  }

  function promptElevation(customReason) {
    showModal({
      title: 'Permisos de Administrador Requeridos',
      message:
        customReason ||
        'Pristine está ejecutándose en <strong>Modo Estándar</strong>. Para desactivar servicios del sistema (como <code>DiagTrack</code>) y aplicar directivas protegidas de Windows, se requieren privilegios de Administrador.<br><br>¿Deseas reiniciar Pristine como Administrador ahora mismo?',
      type: 'warning',
      confirmText: 'Reiniciar como Administrador',
      cancelText: 'Cancelar',
      onConfirm: async () => {
        try {
          const success = await invokeNative('request_elevation');
          if (!success) {
            showModal({
              title: 'Elevación Cancelada',
              message: 'La solicitud de UAC fue rechazada o cancelada por el usuario.',
              type: 'info'
            });
          }
        } catch (err) {
          showModal({
            title: 'Error de Elevación',
            message: 'No se pudo iniciar el proceso elevado: ' + err,
            type: 'error'
          });
        }
      }
    });
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
  const elRamUsage = document.getElementById('val-ram-usage');

  const elPrivacyList = document.getElementById('privacy-tweaks-list');
  const elServicesList = document.getElementById('services-list');
  const elRollbackList = document.getElementById('rollback-list');
  const elSelectionCount = document.getElementById('lbl-selection-count');

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

  // Category Filter Tabs Logic
  function setupCategoryFilters() {
    const tabs = document.querySelectorAll('.filter-tab');
    tabs.forEach(tab => {
      tab.addEventListener('click', () => {
        tabs.forEach(t => t.classList.remove('active'));
        tab.classList.add('active');
        state.selectedCategory = tab.getAttribute('data-category') || 'all';
        renderCatalog();
      });
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

  function updateSelectionCountLabel() {
    if (elSelectionCount) {
      elSelectionCount.textContent = `${state.selectedTweakIds.size} seleccionadas`;
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

        if (elRamUsage && metrics.ram_total_mb > 0) {
          const usedGb = (metrics.ram_used_mb / 1024).toFixed(1);
          const totalGb = (metrics.ram_total_mb / 1024).toFixed(1);
          elRamUsage.textContent = `${usedGb} / ${totalGb} GB (${metrics.ram_percent}%)`;
        }
      }
    } catch (_) {}
  }

  // Render Tweak Cards
  function renderTweakCard(tweak, status) {
    const card = document.createElement('div');
    card.className = 'tweak-card';
    card.id = `card-${tweak.id}`;

    const isChecked = state.selectedTweakIds.has(tweak.id);
    if (isChecked) card.classList.add('active-card');

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
          <input type="checkbox" id="toggle-${tweak.id}" ${isChecked ? 'checked' : ''}>
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
      updateSelectionCountLabel();
    });

    return card;
  }

  // Load and Render Views
  async function loadData() {
    try {
      state.catalog = await invokeNative('get_catalog');

      state.audit = await invokeNative('audit_system');
      if (state.audit) {
        updatePrivacyMeter(state.audit.privacy_score_percent);
        elActiveCount.textContent = state.audit.active_count;
        elTotalCount.textContent = state.audit.total_analyzed;

        // Initialize selected tweak IDs from active audit state if empty
        if (state.selectedTweakIds.size === 0) {
          state.audit.statuses.forEach(s => {
            if (s.state === 'active') {
              state.selectedTweakIds.add(s.tweak_id);
            }
          });
        }
      }

      renderCatalog();
      updateSelectionCountLabel();
      await loadTransactionHistory();
    } catch (err) {
      console.error('Error loading catalog and audit:', err);
    }
  }

  function renderCatalog() {
    elPrivacyList.innerHTML = '';
    elServicesList.innerHTML = '';

    const currentFilter = state.selectedCategory;

    state.catalog.forEach(tweak => {
      const status = state.audit ? state.audit.statuses.find(s => s.tweak_id === tweak.id) : null;

      // Filter by category in privacy view
      const matchesFilter = currentFilter === 'all' || tweak.category === currentFilter;
      if (matchesFilter) {
        const card = renderTweakCard(tweak, status);
        elPrivacyList.appendChild(card);
      }

      // Populate services tab with telemetry and latency tweaks
      if (tweak.category === 'telemetry' || tweak.category === 'latency') {
        const srvCard = renderTweakCard(tweak, status);
        elServicesList.appendChild(srvCard);
      }
    });
  }

  // Load Transaction History from Persistent Journal
  async function loadTransactionHistory() {
    try {
      const history = await invokeNative('get_transaction_history');
      if (Array.isArray(history)) {
        state.sessions = history;
        renderRollbackHistory();
      }
    } catch (err) {
      console.error('Error loading transaction history:', err);
    }
  }

  // ============================================================================
  // Software Uninstaller & Bloatware Remover Logic
  // ============================================================================
  async function loadInstalledApps() {
    const listEl = document.getElementById('apps-list');
    if (!listEl) return;

    try {
      const apps = await invokeNative('get_installed_apps');
      if (Array.isArray(apps)) {
        state.apps = apps;

        // Update count badges
        const countAll = document.getElementById('count-all-apps');
        const countBloat = document.getElementById('count-bloatware-apps');
        const countUser = document.getElementById('count-user-apps');
        const countSystem = document.getElementById('count-system-apps');

        if (countAll) countAll.textContent = apps.length;
        if (countBloat) countBloat.textContent = apps.filter(a => a.category === 'bloatware').length;
        if (countUser) countUser.textContent = apps.filter(a => a.category === 'user').length;
        if (countSystem) countSystem.textContent = apps.filter(a => a.category === 'system').length;

        renderApps();
      }
    } catch (err) {
      console.error('Error loading installed apps:', err);
      listEl.innerHTML = `
        <div style="font-size: 13px; color: var(--accent-rose); padding: 24px 0; text-align: center;">
          Error al obtener inventario de programas: ${err}
        </div>
      `;
    }
  }

  function renderApps() {
    const listEl = document.getElementById('apps-list');
    if (!listEl) return;

    listEl.innerHTML = '';

    const filter = state.selectedAppFilter;
    const query = state.appSearchQuery.toLowerCase().trim();

    const filtered = state.apps.filter(app => {
      const matchesFilter = filter === 'all' || app.category === filter;
      const matchesQuery =
        !query ||
        app.name.toLowerCase().includes(query) ||
        app.publisher.toLowerCase().includes(query) ||
        app.id.toLowerCase().includes(query);
      return matchesFilter && matchesQuery;
    });

    if (filtered.length === 0) {
      listEl.innerHTML = `
        <div style="font-size: 13px; color: var(--text-muted); padding: 30px 0; text-align: center;">
          No se encontraron aplicaciones que coincidan con los criterios de búsqueda.
        </div>
      `;
      return;
    }

    filtered.forEach(app => {
      const card = document.createElement('div');
      card.className = 'app-card';

      const badgeClass = app.category === 'bloatware' ? 'bloatware' : app.category === 'system' ? 'system' : 'user';
      const badgeText = app.category === 'bloatware' ? 'Bloatware' : app.category === 'system' ? 'Sistema' : 'Usuario';
      const isSystem = app.category === 'system';

      const sizeStr = app.estimated_size_mb > 0 ? `${app.estimated_size_mb} MB` : (app.is_uwp ? 'UWP App' : 'Tamaño N/D');

      card.innerHTML = `
        <div class="app-info">
          <div class="app-name-row">
            <span class="app-name">${app.name}</span>
            <span class="app-badge ${badgeClass}">${badgeText}</span>
          </div>
          <div class="app-meta">
            <span>${app.publisher}</span>
            <span>&bull;</span>
            <span>v${app.version}</span>
            <span>&bull;</span>
            <span>${sizeStr}</span>
          </div>
        </div>
        <div>
          ${
            isSystem
              ? `<button class="btn btn-secondary btn-uninstall" disabled title="Componente protegido del sistema">Protegido</button>`
              : `<button class="btn btn-uninstall btn-danger btn-uninstall-action" data-id="${app.id}">
                  <svg viewBox="0 0 24 24"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>
                  <span>Desinstalar</span>
                </button>`
          }
        </div>
      `;

      const btnUninstall = card.querySelector('.btn-uninstall-action');
      if (btnUninstall) {
        btnUninstall.addEventListener('click', () => {
          if (!state.isElevated) {
            promptElevation('Desinstalar aplicaciones del sistema requiere permisos de Administrador.<br><br>¿Deseas reiniciar Pristine como Administrador?');
            return;
          }

          showModal({
            title: `¿Desinstalar ${app.name}?`,
            message: `Estás a punto de desinstalar <strong>${app.name}</strong> (${app.version}) de <em>${app.publisher}</em>.<br><br>¿Deseas proceder con la eliminación segura?`,
            type: app.category === 'bloatware' ? 'info' : 'warning',
            confirmText: 'Desinstalar Ahora',
            cancelText: 'Cancelar',
            onConfirm: async () => {
              try {
                const res = await invokeNative('uninstall_app', {
                  appId: app.id,
                  isUwp: app.is_uwp,
                  uninstallCmd: app.uninstall_cmd
                });

                showModal({
                  title: res.success ? 'Desinstalación Finalizada' : 'Aviso',
                  message: res.message || 'Proceso completado.',
                  type: res.success ? 'info' : 'warning'
                });

                await loadInstalledApps();
              } catch (err) {
                showModal({
                  title: 'Error al Desinstalar',
                  message: 'Fallo al desinstalar la aplicación: ' + err,
                  type: 'error'
                });
              }
            }
          });
        });
      }

      listEl.appendChild(card);
    });
  }

  // ============================================================================
  // Hosts Telemetry Shield Status
  // ============================================================================
  async function checkHostsShieldStatus() {
    try {
      const active = await invokeNative('get_hosts_shield_status');
      state.hostsShieldActive = !!active;

      const checkbox = document.getElementById('toggle-hosts-shield');
      const statusLabel = document.getElementById('hosts-shield-status');

      if (checkbox) checkbox.checked = state.hostsShieldActive;
      if (statusLabel) {
        if (state.hostsShieldActive) {
          statusLabel.textContent = 'ACTIVO: 40+ dominios de telemetría redirigidos a 0.0.0.0 localmente.';
          statusLabel.style.color = 'var(--accent-emerald)';
        } else {
          statusLabel.textContent = 'Inactivo: Las peticiones DNS se resuelven normalmente.';
          statusLabel.style.color = 'var(--text-muted)';
        }
      }
    } catch (e) {
      console.warn('Could not check hosts shield status:', e);
    }
  }

  // Setup Action Handlers
  function setupActions() {
    // Privilege Elevation Badge & Banner Handlers
    const badge = document.getElementById('privilege-badge');
    if (badge) {
      badge.addEventListener('click', () => {
        if (!state.isElevated) {
          promptElevation();
        }
      });
    }

    const btnBannerElevate = document.getElementById('btn-banner-elevate');
    if (btnBannerElevate) {
      btnBannerElevate.addEventListener('click', () => {
        promptElevation();
      });
    }

    // Quick Scan
    document.getElementById('btn-quick-scan').addEventListener('click', async () => {
      const btn = document.getElementById('btn-quick-scan');
      btn.disabled = true;
      btn.querySelector('span').textContent = 'Escaneando...';

      await loadData();

      btn.disabled = false;
      btn.querySelector('span').textContent = 'Escanear Sistema';
    });

    // Create VSS System Restore Point
    const btnRestorePoint = document.getElementById('btn-create-restore-point');
    if (btnRestorePoint) {
      btnRestorePoint.addEventListener('click', async () => {
        if (!state.isElevated) {
          promptElevation('La creación de puntos de restauración VSS de Windows requiere permisos de Administrador.<br><br>¿Deseas reiniciar Pristine como Administrador?');
          return;
        }

        btnRestorePoint.disabled = true;
        const span = btnRestorePoint.querySelector('span');
        const prevText = span.textContent;
        span.textContent = 'Creando Snapshot VSS...';

        try {
          const res = await invokeNative('create_system_restore_point');
          showModal({
            title: res.success ? 'Snapshot VSS Creado' : 'Aviso del Sistema VSS',
            message: res.message || 'Punto de restauración del sistema creado con éxito.',
            type: res.success ? 'info' : 'warning'
          });
        } catch (err) {
          showModal({
            title: 'Error al Crear Snapshot',
            message: 'Ocurrió un error al crear el punto de restauración VSS: ' + err,
            type: 'error'
          });
        } finally {
          btnRestorePoint.disabled = false;
          span.textContent = prevText;
        }
      });
    }

    // Apply Recommended Tweaks
    document.getElementById('btn-apply-recommended').addEventListener('click', () => {
      state.catalog.forEach(tweak => {
        if (tweak.default_recommended) {
          state.selectedTweakIds.add(tweak.id);
        }
      });
      renderCatalog();
      updateSelectionCountLabel();
    });

    // Save Privacy Changes
    document.getElementById('btn-save-privacy').addEventListener('click', async () => {
      // Check if user is in standard mode and attempting to apply tweaks
      if (!state.isElevated) {
        promptElevation('Pristine se encuentra en <strong>Modo Estándar</strong>. La configuración de servicios del sistema (como <code>DiagTrack</code>) y directivas de privacidad de Windows requiere permisos de Administrador.<br><br>¿Deseas reiniciar Pristine como Administrador ahora mismo para aplicar estos cambios?');
        return;
      }

      const btn = document.getElementById('btn-save-privacy');
      btn.disabled = true;
      btn.querySelector('span').textContent = 'Aplicando...';

      try {
        const session = await invokeNative('apply_tweaks', {
          tweakIds: Array.from(state.selectedTweakIds),
          description: 'Ajuste de directivas de privacidad y optimización'
        });

        if (session) {
          state.sessions.unshift(session);
          renderRollbackHistory();
          showModal({
            title: 'Directivas Aplicadas con Éxito',
            message: `Se aplicaron <strong>${state.selectedTweakIds.size} optimizaciones</strong> de forma segura.<br>Se generó una sesión transaccional firmada (Hash: <code>${session.integrity_hash.substring(0, 16)}...</code>). Puedes revertir estos cambios en 1-clic desde la pestaña de Rollback en cualquier momento.`,
            type: 'info'
          });
        }

        await loadData();
      } catch (err) {
        console.error('Error applying tweaks:', err);
        const errStr = String(err);
        if (errStr.includes('0x80070005') || errStr.includes('Access is denied') || errStr.includes('Elevated Administrator') || errStr.includes('privileges required')) {
          promptElevation('<strong>Permisos Insuficientes (Acceso Denegado 0x80070005):</strong><br>Windows impidió modificar los servicios o directivas seleccionadas porque Pristine no se está ejecutando como Administrador.<br><br>¿Deseas reiniciar Pristine con elevación UAC ahora?');
        } else {
          showModal({
            title: 'Error al Aplicar Cambios',
            message: 'Ocurrió un error al aplicar las directivas: ' + errStr,
            type: 'error'
          });
        }
      } finally {
        btn.disabled = false;
        btn.querySelector('span').textContent = 'Aplicar Cambios';
      }
    });

    // Safe Temporary Files Cleaner
    const btnClean = document.getElementById('btn-run-cleaner');
    const elCleanStatus = document.getElementById('clean-status');
    const btnQuickClean = document.getElementById('btn-quick-clean');

    async function runCleaning() {
      btnClean.disabled = true;
      if (btnQuickClean) btnQuickClean.disabled = true;
      elCleanStatus.textContent = 'Analizando y purgando archivos seguros...';

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
        if (btnQuickClean) btnQuickClean.disabled = false;
      }
    }

    btnClean.addEventListener('click', runCleaning);
    if (btnQuickClean) btnQuickClean.addEventListener('click', runCleaning);

    // WinSxS Component Store Cleaner (DISM)
    const btnDism = document.getElementById('btn-run-dism');
    const elDismStatus = document.getElementById('dism-status');

    if (btnDism) {
      btnDism.addEventListener('click', async () => {
        btnDism.disabled = true;
        elDismStatus.textContent = 'Ejecutando DISM /StartComponentCleanup (puede tardar unos minutos)...';

        try {
          const out = await invokeNative('clean_winsxs_component_store');
          elDismStatus.textContent = out || 'Limpieza de almacén WinSxS completada con éxito.';
        } catch (err) {
          elDismStatus.textContent = 'Error al ejecutar DISM: ' + err;
        } finally {
          btnDism.disabled = false;
        }
      });
    }

    // Refresh Rollback History Button
    const btnRefreshRollback = document.getElementById('btn-refresh-rollback');
    if (btnRefreshRollback) {
      btnRefreshRollback.addEventListener('click', async () => {
        btnRefreshRollback.disabled = true;
        await loadTransactionHistory();
        btnRefreshRollback.disabled = false;
      });
    }

    // Flush DNS Resolver Cache Button
    const btnFlushDns = document.getElementById('btn-flush-dns');
    if (btnFlushDns) {
      btnFlushDns.addEventListener('click', async () => {
        btnFlushDns.disabled = true;
        try {
          const res = await invokeNative('flush_dns');
          showModal({
            title: 'Caché DNS Purgada',
            message: res.message || 'La caché de resolución DNS de Windows ha sido vaciada con éxito.',
            type: 'info'
          });
        } catch (err) {
          showModal({
            title: 'Error de Red',
            message: 'Error al purgar la caché DNS: ' + err,
            type: 'error'
          });
        } finally {
          btnFlushDns.disabled = false;
        }
      });
    }

    // Restart Windows Explorer Button
    const btnRestartExp = document.getElementById('btn-restart-explorer');
    if (btnRestartExp) {
      btnRestartExp.addEventListener('click', async () => {
        btnRestartExp.disabled = true;
        try {
          await invokeNative('restart_windows_explorer');
          showModal({
            title: 'Explorador Reiniciado',
            message: 'El proceso explorer.exe se ha reiniciado correctamente. La barra de tareas y el menú de inicio han sido refrescados.',
            type: 'info'
          });
        } catch (err) {
          showModal({
            title: 'Error de Shell',
            message: 'Error al reiniciar explorer.exe: ' + err,
            type: 'error'
          });
        } finally {
          btnRestartExp.disabled = false;
        }
      });
    }

    // Hosts Telemetry Shield Toggle
    const toggleHosts = document.getElementById('toggle-hosts-shield');
    if (toggleHosts) {
      toggleHosts.addEventListener('change', async () => {
        if (!state.isElevated) {
          toggleHosts.checked = !toggleHosts.checked;
          promptElevation('Modificar el archivo hosts de Windows requiere permisos de Administrador.<br><br>¿Deseas reiniciar Pristine como Administrador?');
          return;
        }

        const target = toggleHosts.checked;
        try {
          const res = await invokeNative('toggle_hosts_shield', { enable: target });
          showModal({
            title: target ? 'Escudo Hosts Activado' : 'Escudo Hosts Desactivado',
            message: res.message || 'Archivo hosts actualizado y caché DNS purgada.',
            type: 'info'
          });
          await checkHostsShieldStatus();
        } catch (err) {
          toggleHosts.checked = !target;
          showModal({
            title: 'Error en Archivo Hosts',
            message: 'Error al modificar archivo hosts: ' + err,
            type: 'error'
          });
        }
      });
    }

    // Refresh Installed Apps Button
    const btnRefreshApps = document.getElementById('btn-refresh-apps');
    if (btnRefreshApps) {
      btnRefreshApps.addEventListener('click', async () => {
        btnRefreshApps.disabled = true;
        await loadInstalledApps();
        btnRefreshApps.disabled = false;
      });
    }

    // Apps Real-time Search Input
    const inputSearchApps = document.getElementById('input-search-apps');
    if (inputSearchApps) {
      inputSearchApps.addEventListener('input', e => {
        state.appSearchQuery = e.target.value;
        renderApps();
      });
    }

    // Apps Filter Tabs
    document.querySelectorAll('[data-app-filter]').forEach(tab => {
      tab.addEventListener('click', () => {
        document.querySelectorAll('[data-app-filter]').forEach(t => t.classList.remove('active'));
        tab.classList.add('active');
        state.selectedAppFilter = tab.getAttribute('data-app-filter') || 'all';
        renderApps();
      });
    });
  }

  // Render Rollback Timeline
  function renderRollbackHistory() {
    if (!elRollbackList) return;

    if (state.sessions.length === 0) {
      elRollbackList.innerHTML = `
        <div style="font-size: 13px; color: var(--text-muted); padding: 12px 0;">
          No hay transacciones previas registradas. Aplica cambios para generar sesiones de rollback.
        </div>
      `;
      return;
    }

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

      item.querySelector('.btn-revert').addEventListener('click', () => {
        showModal({
          title: '¿Revertir Sesión Transaccional?',
          message: `¿Deseas revertir la sesión '<strong>${session.description}</strong>' al estado exacto previo?<br><br>Todas las claves del registro, servicios y tareas programadas serán restaurados a sus valores originales con garantía criptográfica.`,
          type: 'warning',
          confirmText: 'Revertir Ahora',
          cancelText: 'Cancelar',
          onConfirm: async () => {
            try {
              await invokeNative('revert_transaction', { session });
              showModal({
                title: 'Sesión Revertida',
                message: 'La sesión se revirtió exitosamente. Los valores previos han sido restaurados.',
                type: 'info'
              });
              await loadTransactionHistory();
              await loadData();
            } catch (err) {
              showModal({
                title: 'Error en Reversión',
                message: 'Error al revertir sesión: ' + err,
                type: 'error'
              });
            }
          }
        });
      });

      elRollbackList.appendChild(item);
    });
  }

  // App Initialization
  async function initApp() {
    console.log('[PRISTINE] Initializing UI application...');
    setupNavigation();
    setupThemeHandlers();
    setupCategoryFilters();
    setupActions();
    await updatePrivilegeState();
    await checkHostsShieldStatus();
    loadData();
    loadInstalledApps();

    // Start live metrics loop
    pollHardwareMetrics();
    setInterval(pollHardwareMetrics, 1500);

    invokeNative('frontend_log', { msg: 'DOM, category filters, apps inventory, privilege state and handlers initialized successfully' }).catch(() => {});
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initApp);
  } else {
    initApp();
  }
})();
