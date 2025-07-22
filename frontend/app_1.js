// SmartFix Core - Frontend Dashboard Application (Final Fixed Version)
console.log('🚀 SmartFix Core loading...');

class SmartFixApp {
    constructor() {
        this.currentUser = null;
        this.jwtToken = null;
        this.websocket = null;
        this.currentRoute = 'dashboard';
        this.isWebSocketConnected = false;
        
        // Mock data
        this.mockData = {
            users: [
                { id: 1, username: 'admin', role: 'administrator', email: 'admin@smartfix.core' },
                { id: 2, username: 'developer', role: 'developer', email: 'dev@smartfix.core' },
                { id: 3, username: 'viewer', role: 'viewer', email: 'viewer@smartfix.core' }
            ],
            queueJobs: [
                { id: 'job-001', name: 'WhatsApp-v2.23.apk', status: 'processing', progress: 67, priority: 'high', eta: '2m 15s' },
                { id: 'job-002', name: 'Instagram-v295.apk', status: 'completed', progress: 100, priority: 'normal', eta: '0m' },
                { id: 'job-003', name: 'TikTok-v28.apk', status: 'queued', progress: 0, priority: 'low', eta: '8m 30s' },
                { id: 'job-004', name: 'Spotify-v8.7.apk', status: 'failed', progress: 25, priority: 'high', eta: 'N/A' }
            ],
            patches: [
                { id: 1, filename: 'security-fix-v1.2.apk', status: 'completed', uploadDate: '2025-01-20T10:30:00Z', size: 12456789 },
                { id: 2, filename: 'ui-update-v2.1.aab', status: 'processing', uploadDate: '2025-01-21T09:15:00Z', size: 8934512 }
            ],
            plugins: [
                { id: 'security-scanner', name: 'Security Scanner Pro', description: 'Advanced vulnerability detection', version: '2.1.0', rating: 4.8, downloads: 15420, category: 'Security', installed: true },
                { id: 'ui-optimizer', name: 'UI Performance Optimizer', description: 'Optimize Android UI performance', version: '1.5.2', rating: 4.6, downloads: 8930, category: 'Performance', installed: false }
            ],
            metrics: { queueSize: 7, successRate: 95.2, avgLatency: 145, activeWorkers: 3, totalProcessed: 1322 }
        };

        console.log('SmartFixApp constructed');
        this.initializeWhenReady();
    }

    initializeWhenReady() {
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', () => this.initialize());
        } else {
            this.initialize();
        }
    }

    initialize() {
        console.log('🔧 Initializing SmartFix Core...');
        
        try {
            this.showLogin();
            this.setupEventListeners();
            this.setupModal();
            this.applyInitialTheme();
            console.log('✅ SmartFix Core initialized successfully');
        } catch (error) {
            console.error('❌ Initialization error:', error);
        }
    }

    showLogin() {
        const loginScreen = document.getElementById('login-screen');
        const mainApp = document.getElementById('main-app');
        
        if (loginScreen) {
            loginScreen.classList.remove('hidden');
            console.log('Login screen shown');
        }
        if (mainApp) {
            mainApp.classList.add('hidden');
        }
    }

    showMainApp() {
        const loginScreen = document.getElementById('login-screen');
        const mainApp = document.getElementById('main-app');
        
        if (loginScreen) loginScreen.classList.add('hidden');
        if (mainApp) {
            mainApp.classList.remove('hidden');
            console.log('Main app shown');
        }
        
        this.updateUserInfo();
        this.navigateToRoute('dashboard');
        this.initializePages();
        this.startTimers();
    }

    setupEventListeners() {
        console.log('Setting up event listeners...');

        // Login form
        const loginForm = document.getElementById('login-form');
        if (loginForm) {
            loginForm.addEventListener('submit', (e) => {
                e.preventDefault();
                e.stopPropagation();
                console.log('Login form submitted');
                this.handleLogin();
            });
            console.log('Login form listener added');
        } else {
            console.error('Login form not found');
        }

        // Navigation items
        const navItems = document.querySelectorAll('.nav-item');
        navItems.forEach((item, index) => {
            item.addEventListener('click', (e) => {
                e.preventDefault();
                const route = item.getAttribute('data-route');
                console.log(`Navigation clicked: ${route}`);
                this.navigateToRoute(route);
            });
        });
        console.log(`Added ${navItems.length} navigation listeners`);

        // Logout button
        const logoutBtn = document.getElementById('logout-btn');
        if (logoutBtn) {
            logoutBtn.addEventListener('click', () => {
                console.log('Logout clicked');
                this.handleLogout();
            });
        }

        // Theme toggle
        const themeBtn = document.getElementById('theme-toggle');
        if (themeBtn) {
            themeBtn.addEventListener('click', () => this.toggleTheme());
        }

        // Queue refresh
        const refreshBtn = document.getElementById('refresh-queue');
        if (refreshBtn) {
            refreshBtn.addEventListener('click', () => {
                console.log('Queue refresh clicked');
                this.refreshQueue();
            });
        }

        // Status filter
        const statusFilter = document.getElementById('status-filter');
        if (statusFilter) {
            statusFilter.addEventListener('change', (e) => this.filterQueue(e.target.value));
        }

        console.log('Event listeners setup complete');
    }

    async handleLogin() {
        console.log('🔐 Login process started');
        
        const usernameSelect = document.getElementById('username-select');
        const passwordInput = document.getElementById('password-input');
        const submitBtn = document.querySelector('#login-form button[type="submit"]');
        
        if (!usernameSelect || !passwordInput || !submitBtn) {
            console.error('Login form elements missing');
            this.showNotification('Błąd formularza logowania', 'error');
            return;
        }

        const username = usernameSelect.value;
        const password = passwordInput.value;
        
        console.log('Login attempt:', { username, hasPassword: !!password });

        if (!username) {
            this.showNotification('Wybierz użytkownika', 'error');
            return;
        }

        if (!password) {
            this.showNotification('Wprowadź hasło', 'error');
            return;
        }

        // Set loading state
        const btnText = submitBtn.querySelector('.btn-text');
        const btnLoading = submitBtn.querySelector('.btn-loading');
        
        submitBtn.disabled = true;
        if (btnText) btnText.style.display = 'none';
        if (btnLoading) btnLoading.style.display = 'inline';

        try {
            // Simulate API delay
            await new Promise(resolve => setTimeout(resolve, 1000));

            const user = this.mockData.users.find(u => u.username === username);
            
            console.log('User found:', !!user);
            console.log('Password correct:', password === 'demo123');
            
            if (user && password === 'demo123') {
                this.currentUser = user;
                this.jwtToken = this.generateMockJWT(user);
                
                console.log('✅ Login successful for:', user.username);
                this.showNotification('Logowanie pomyślne!', 'success');
                
                // Short delay then show main app
                setTimeout(() => {
                    this.showMainApp();
                }, 800);
                
            } else {
                console.log('❌ Login failed - invalid credentials');
                this.showNotification('Nieprawidłowe dane. Użyj: admin/developer/viewer + hasło: demo123', 'error');
            }
        } catch (error) {
            console.error('Login error:', error);
            this.showNotification('Błąd logowania', 'error');
        } finally {
            // Reset button state
            setTimeout(() => {
                submitBtn.disabled = false;
                if (btnText) btnText.style.display = 'inline';
                if (btnLoading) btnLoading.style.display = 'none';
            }, 500);
        }
    }

    handleLogout() {
        console.log('🚪 Logout process');
        
        this.currentUser = null;
        this.jwtToken = null;
        
        if (this.websocket) {
            this.websocket.close();
            this.websocket = null;
        }
        
        this.showNotification('Wylogowano', 'success');
        this.showLogin();
        
        // Reset login form
        const usernameSelect = document.getElementById('username-select');
        const passwordInput = document.getElementById('password-input');
        if (usernameSelect) usernameSelect.value = '';
        if (passwordInput) passwordInput.value = 'demo123';
    }

    generateMockJWT(user) {
        const header = { alg: 'HS256', typ: 'JWT' };
        const payload = { sub: user.username, role: user.role, exp: Math.floor(Date.now() / 1000) + 3600 };
        return btoa(JSON.stringify(header)) + '.' + btoa(JSON.stringify(payload)) + '.mock-signature';
    }

    updateUserInfo() {
        if (!this.currentUser) return;
        
        const userNameEl = document.getElementById('user-name');
        const userRoleEl = document.getElementById('user-role');
        const userAvatarEl = document.getElementById('user-avatar');
        
        if (userNameEl) userNameEl.textContent = this.currentUser.username;
        if (userRoleEl) userRoleEl.textContent = this.currentUser.role;
        if (userAvatarEl) userAvatarEl.textContent = this.currentUser.username.charAt(0).toUpperCase();
    }

    navigateToRoute(route) {
        console.log('🧭 Navigating to:', route);
        
        this.currentRoute = route;
        
        // Update nav active state
        document.querySelectorAll('.nav-item').forEach(item => {
            item.classList.toggle('active', item.getAttribute('data-route') === route);
        });
        
        // Update page visibility
        document.querySelectorAll('.page').forEach(page => {
            page.classList.toggle('active', page.id === `${route}-page`);
        });
        
        // Update header
        const titles = {
            dashboard: 'Dashboard',
            queue: 'Monitor Kolejki', 
            patches: 'Zarządzanie Patches',
            traces: 'Trace Explorer',
            marketplace: 'Plugin Marketplace',
            settings: 'Ustawienia'
        };
        
        const pageTitle = document.getElementById('page-title');
        const breadcrumb = document.getElementById('breadcrumb-current');
        
        if (pageTitle) pageTitle.textContent = titles[route] || route;
        if (breadcrumb) breadcrumb.textContent = titles[route] || route;
        
        // Initialize page-specific content
        this.initializePage(route);
    }

    initializePage(route) {
        switch (route) {
            case 'dashboard':
                this.renderMetrics();
                this.renderHeatmap();
                this.renderRecentActivity();
                break;
            case 'queue':
                this.renderQueueTable();
                break;
            case 'patches':
                this.renderPatchesTable();
                break;
            case 'marketplace':
                this.renderPluginsGrid();
                break;
            case 'traces':
                this.initializeTracesPage();
                break;
            case 'settings':
                this.initializeSettingsPage();
                break;
        }
    }

    initializePages() {
        // Initialize all page functionality
        this.initializeDashboard();
        this.initializeQueue();
        this.initializePatches();
        this.initializeMarketplace();
        this.initializeTraces();
        this.initializeSettings();
    }

    // Dashboard functionality
    initializeDashboard() {
        this.renderMetrics();
        this.renderHeatmap();
        this.renderRecentActivity();
    }

    renderMetrics() {
        const m = this.mockData.metrics;
        const updates = {
            'queue-size': m.queueSize,
            'success-rate': m.successRate.toFixed(1) + '%',
            'avg-latency': m.avgLatency + 'ms',
            'active-workers': m.activeWorkers
        };
        
        Object.entries(updates).forEach(([id, value]) => {
            const el = document.getElementById(id);
            if (el) el.textContent = value;
        });
    }

    renderHeatmap() {
        const container = document.getElementById('queue-heatmap');
        if (!container) return;
        
        container.innerHTML = '';
        const colors = ['#00F5FF', '#FF3366', '#FFFF00'];
        
        for (let i = 0; i < 50; i++) {
            const cell = document.createElement('div');
            cell.className = 'heatmap-cell';
            cell.style.backgroundColor = colors[Math.floor(Math.random() * 3)];
            cell.style.opacity = Math.random() * 0.7 + 0.3;
            container.appendChild(cell);
        }
    }

    renderRecentActivity() {
        const activities = [
            { icon: '✓', cls: 'success', text: 'WhatsApp patch ukończony', time: '2 min temu' },
            { icon: '⏳', cls: 'warning', text: 'Instagram patch rozpoczęty', time: '5 min temu' },
            { icon: '⚠️', cls: 'error', text: 'Spotify patch błąd', time: '8 min temu' }
        ];
        
        const container = document.getElementById('recent-activity');
        if (!container) return;
        
        container.innerHTML = activities.map(act => `
            <div class="activity-item">
                <div class="activity-icon status--${act.cls}">${act.icon}</div>
                <div class="activity-content">
                    <div class="activity-title">${act.text}</div>
                    <div class="activity-time">${act.time}</div>
                </div>
            </div>
        `).join('');
    }

    // Queue functionality
    initializeQueue() {
        this.renderQueueTable();
    }

    renderQueueTable() {
        const tbody = document.getElementById('queue-table-body');
        if (!tbody) return;
        
        const statusClasses = { processing: 'warning', queued: 'info', completed: 'success', failed: 'error' };
        const statusTexts = { processing: 'Przetwarzanie', queued: 'W kolejce', completed: 'Ukończone', failed: 'Nieudane' };
        const priorityClasses = { high: 'error', normal: 'warning', low: 'info' };
        const priorityTexts = { high: 'Wysokie', normal: 'Średnie', low: 'Niskie' };
        
        tbody.innerHTML = this.mockData.queueJobs.map(job => `
            <div class="table-row">
                <div class="table-cell">${job.id}</div>
                <div class="table-cell">${job.name}</div>
                <div class="table-cell"><span class="status status--${statusClasses[job.status]}">${statusTexts[job.status]}</span></div>
                <div class="table-cell">
                    <div class="progress-bar">
                        <div class="progress-fill" style="width: ${job.progress}%"></div>
                        <span class="progress-text">${job.progress}%</span>
                    </div>
                </div>
                <div class="table-cell"><span class="status status--${priorityClasses[job.priority]}">${priorityTexts[job.priority]}</span></div>
                <div class="table-cell">${job.eta}</div>
                <div class="table-cell"><button class="btn btn--sm btn--outline" onclick="app.showJobDetails('${job.id}')">Szczegóły</button></div>
            </div>
        `).join('');
    }

    refreshQueue() {
        console.log('🔄 Refreshing queue data');
        
        // Simulate progress updates
        this.mockData.queueJobs.forEach(job => {
            if (job.status === 'processing' && job.progress < 100) {
                job.progress = Math.min(100, job.progress + Math.floor(Math.random() * 20) + 10);
                if (job.progress >= 100) {
                    job.status = 'completed';
                    job.eta = '0m';
                }
            }
        });
        
        this.renderQueueTable();
        this.showNotification('Dane kolejki odświeżone', 'success');
    }

    filterQueue(status) {
        const jobs = status ? this.mockData.queueJobs.filter(j => j.status === status) : this.mockData.queueJobs;
        
        const tbody = document.getElementById('queue-table-body');
        if (!tbody) return;
        
        // Re-render with filtered jobs
        const statusClasses = { processing: 'warning', queued: 'info', completed: 'success', failed: 'error' };
        const statusTexts = { processing: 'Przetwarzanie', queued: 'W kolejce', completed: 'Ukończone', failed: 'Nieudane' };
        
        tbody.innerHTML = jobs.map(job => `
            <div class="table-row">
                <div class="table-cell">${job.id}</div>
                <div class="table-cell">${job.name}</div>
                <div class="table-cell"><span class="status status--${statusClasses[job.status]}">${statusTexts[job.status]}</span></div>
                <div class="table-cell">
                    <div class="progress-bar">
                        <div class="progress-fill" style="width: ${job.progress}%"></div>
                        <span class="progress-text">${job.progress}%</span>
                    </div>
                </div>
                <div class="table-cell">${job.priority}</div>
                <div class="table-cell">${job.eta}</div>
                <div class="table-cell"><button class="btn btn--sm btn--outline">Szczegóły</button></div>
            </div>
        `).join('');
    }

    showJobDetails(jobId) {
        const job = this.mockData.queueJobs.find(j => j.id === jobId);
        if (!job) return;
        
        this.showModal(`Szczegóły ${jobId}`, `
            <p><strong>Nazwa:</strong> ${job.name}</p>
            <p><strong>Status:</strong> ${job.status}</p>
            <p><strong>Postęp:</strong> ${job.progress}%</p>
            <p><strong>Priority:</strong> ${job.priority}</p>
            <p><strong>ETA:</strong> ${job.eta}</p>
        `);
    }

    // Patches functionality
    initializePatches() {
        this.renderPatchesTable();
    }

    renderPatchesTable() {
        const tbody = document.getElementById('patches-table-body');
        if (!tbody) return;
        
        tbody.innerHTML = this.mockData.patches.map(patch => `
            <div class="table-row">
                <div class="table-cell">${patch.filename}</div>
                <div class="table-cell"><span class="status status--${patch.status === 'completed' ? 'success' : 'warning'}">${patch.status}</span></div>
                <div class="table-cell">${new Date(patch.uploadDate).toLocaleDateString('pl-PL')}</div>
                <div class="table-cell">${(patch.size / 1024 / 1024).toFixed(2)} MB</div>
                <div class="table-cell"><button class="btn btn--sm btn--outline">Pobierz</button></div>
            </div>
        `).join('');
    }

    // Marketplace functionality
    initializeMarketplace() {
        this.renderPluginsGrid();
    }

    renderPluginsGrid() {
        const grid = document.getElementById('plugins-grid');
        if (!grid) return;
        
        grid.innerHTML = this.mockData.plugins.map(plugin => `
            <div class="plugin-card">
                <div class="plugin-header">
                    <div>
                        <div class="plugin-title">${plugin.name}</div>
                        <div class="plugin-version">v${plugin.version}</div>
                    </div>
                    <div class="plugin-status plugin-status--${plugin.installed ? 'installed' : 'available'}">
                        ${plugin.installed ? 'Zainstalowany' : 'Dostępny'}
                    </div>
                </div>
                <div class="plugin-description">${plugin.description}</div>
                <div class="plugin-meta">
                    <div class="plugin-rating">
                        <span>${'★'.repeat(Math.floor(plugin.rating))}${'☆'.repeat(5 - Math.floor(plugin.rating))}</span>
                        <span>${plugin.rating}</span>
                    </div>
                    <div>${plugin.downloads.toLocaleString()} pobrań</div>
                </div>
                <div class="plugin-actions">
                    <button class="btn btn--${plugin.installed ? 'secondary' : 'primary'} btn--sm">
                        ${plugin.installed ? 'Odinstaluj' : 'Instaluj'}
                    </button>
                    <button class="btn btn--outline btn--sm">Szczegóły</button>
                </div>
            </div>
        `).join('');
    }

    // Traces functionality
    initializeTraces() {
        const searchBtn = document.getElementById('search-traces-btn');
        if (searchBtn) {
            searchBtn.addEventListener('click', () => this.searchTraces());
        }
    }

    searchTraces() {
        const container = document.getElementById('trace-visualization');
        if (!container) return;
        
        container.innerHTML = `
            <div class="trace-results">
                <h4>Wyniki wyszukiwania</h4>
                <p>Znaleziono trace data (symulacja)</p>
                <div style="background: rgba(255,255,255,0.1); padding: 16px; border-radius: 8px; margin-top: 16px;">
                    <p><strong>Trace ID:</strong> trace-${Math.random().toString(36).substr(2,8)}</p>
                    <p><strong>Status:</strong> <span class="status status--success">Sukces</span></p>
                    <p><strong>Czas trwania:</strong> 245ms</p>
                </div>
            </div>
        `;
    }

    // Settings functionality
    initializeSettings() {
        // Settings navigation
        document.querySelectorAll('.settings-nav-item').forEach(item => {
            item.addEventListener('click', (e) => {
                document.querySelectorAll('.settings-nav-item').forEach(i => i.classList.remove('active'));
                e.target.classList.add('active');
                
                const section = e.target.getAttribute('data-section');
                document.querySelectorAll('.settings-section').forEach(s => s.classList.remove('active'));
                const targetSection = document.getElementById(`${section}-settings`);
                if (targetSection) targetSection.classList.add('active');
            });
        });
        
        // Save buttons
        ['save-profile', 'save-api', 'save-notifications', 'save-security'].forEach(btnId => {
            const btn = document.getElementById(btnId);
            if (btn) {
                btn.addEventListener('click', () => {
                    this.showNotification('Ustawienia zapisane', 'success');
                });
            }
        });
    }

    // Theme functionality
    toggleTheme() {
        const html = document.documentElement;
        const current = html.getAttribute('data-color-scheme');
        const newTheme = current === 'dark' ? 'light' : 'dark';
        
        html.setAttribute('data-color-scheme', newTheme);
        
        const themeBtn = document.getElementById('theme-toggle');
        if (themeBtn) themeBtn.textContent = newTheme === 'dark' ? '☀️' : '🌙';
        
        this.showNotification(`Motyw zmieniony na ${newTheme === 'dark' ? 'ciemny' : 'jasny'}`, 'info');
    }

    applyInitialTheme() {
        document.documentElement.setAttribute('data-color-scheme', 'dark');
        const themeBtn = document.getElementById('theme-toggle');
        if (themeBtn) themeBtn.textContent = '☀️';
    }

    // Timers
    startTimers() {
        console.log('⏰ Starting update timers');
        
        // Update dashboard metrics every 6 seconds
        setInterval(() => {
            if (this.currentRoute === 'dashboard') {
                const m = this.mockData.metrics;
                m.queueSize += Math.floor(Math.random() * 3) - 1;
                m.queueSize = Math.max(0, Math.min(15, m.queueSize));
                this.renderMetrics();
            }
        }, 6000);
    }

    // Notification system
    showNotification(message, type = 'info') {
        console.log(`📢 Notification: ${message} (${type})`);
        
        const container = document.getElementById('notifications-container');
        if (!container) return;
        
        const notification = document.createElement('div');
        notification.className = `notification ${type}`;
        
        const typeLabels = { error: 'Błąd', warning: 'Ostrzeżenie', success: 'Sukces', info: 'Info' };
        
        notification.innerHTML = `
            <div class="notification-content">
                <strong>${typeLabels[type]}</strong>
                <p>${message}</p>
            </div>
        `;
        
        container.appendChild(notification);
        
        // Auto remove
        setTimeout(() => notification.remove(), 4000);
        
        // Click to remove
        notification.addEventListener('click', () => notification.remove());
    }

    // Modal system
    setupModal() {
        const overlay = document.getElementById('modal-overlay');
        const closeBtn = document.getElementById('modal-close');
        const cancelBtn = document.getElementById('modal-cancel');
        
        const closeModal = () => {
            if (overlay) overlay.classList.add('hidden');
        };
        
        if (closeBtn) closeBtn.addEventListener('click', closeModal);
        if (cancelBtn) cancelBtn.addEventListener('click', closeModal);
        if (overlay) {
            overlay.addEventListener('click', (e) => {
                if (e.target === overlay) closeModal();
            });
        }
        
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape') closeModal();
        });
    }

    showModal(title, content) {
        const overlay = document.getElementById('modal-overlay');
        const titleEl = document.getElementById('modal-title');
        const bodyEl = document.getElementById('modal-body');
        const confirmBtn = document.getElementById('modal-confirm');
        
        if (!overlay || !titleEl || !bodyEl) return;
        
        titleEl.textContent = title;
        bodyEl.innerHTML = content;
        if (confirmBtn) confirmBtn.style.display = 'none';
        
        overlay.classList.remove('hidden');
    }
}

// Initialize when script loads
console.log('📱 Creating SmartFixApp instance...');
window.app = new SmartFixApp();
console.log('🎯 SmartFix Core ready!');