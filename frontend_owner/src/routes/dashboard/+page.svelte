<script lang="ts">
    // Mock data for initial UI
    let companies = $state([
        { id: '1', name: 'Acme Corp', code: 'ACME', status: 'Active', plan: 'Enterprise' },
        { id: '2', name: 'Global Tech', code: 'GTECH', status: 'Active', plan: 'Professional' },
        { id: '3', name: 'Small Biz Inc', code: 'SBI', status: 'Suspended', plan: 'Basic' }
    ]);

    let stats = $state({
        totalCompanies: 124,
        activeUsers: 4502,
        systemHealth: 'Healthy'
    });
</script>

<div class="dashboard-layout">
    <aside class="sidebar">
        <div class="logo">Owner Console</div>
        <nav>
            <a href="/dashboard" class="active">Overview</a>
            <a href="/dashboard/companies">Companies</a>
            <a href="/dashboard/staff">Staff Management</a>
            <a href="/dashboard/settings">Global Settings</a>
        </nav>
        <div class="footer">
            <button class="logout">Logout</button>
        </div>
    </aside>

    <main class="content">
        <header>
            <h1>System Overview</h1>
            <div class="user-profile">Staff Admin</div>
        </header>

        <div class="stats-grid">
            <div class="stat-card">
                <div class="label">Total Companies</div>
                <div class="value">{stats.totalCompanies}</div>
            </div>
            <div class="stat-card">
                <div class="label">Active Users</div>
                <div class="value">{stats.activeUsers}</div>
            </div>
            <div class="stat-card">
                <div class="label">System Health</div>
                <div class="value status-ok">{stats.systemHealth}</div>
            </div>
        </div>

        <section class="table-section">
            <div class="section-header">
                <h2>Recent Companies</h2>
                <button class="btn-primary">Add New Company</button>
            </div>
            <table>
                <thead>
                    <tr>
                        <th>Company Name</th>
                        <th>Code</th>
                        <th>Plan</th>
                        <th>Status</th>
                        <th>Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {#each companies as company}
                        <tr>
                            <td>{company.name}</td>
                            <td><code>{company.code}</code></td>
                            <td>{company.plan}</td>
                            <td>
                                <span class="badge {company.status.toLowerCase()}">
                                    {company.status}
                                </span>
                            </td>
                            <td>
                                <button class="btn-text">Manage</button>
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </section>
    </main>
</div>

<style>
    :global(body) {
        margin: 0;
        font-family: 'Inter', system-ui, sans-serif;
        background: #f8fafc;
    }

    .dashboard-layout {
        display: flex;
        min-height: 100vh;
    }

    .sidebar {
        width: 260px;
        background: #0f172a;
        color: white;
        display: flex;
        flex-direction: column;
        padding: 2rem 0;
    }

    .logo {
        padding: 0 2rem;
        font-size: 1.25rem;
        font-weight: 800;
        margin-bottom: 3rem;
        color: #3b82f6;
    }

    nav {
        flex: 1;
    }

    nav a {
        display: block;
        padding: 0.75rem 2rem;
        color: #94a3b8;
        text-decoration: none;
        transition: all 0.2s;
        border-left: 4px solid transparent;
    }

    nav a:hover {
        background: rgba(255, 255, 255, 0.05);
        color: white;
    }

    nav a.active {
        background: rgba(59, 130, 246, 0.1);
        color: white;
        border-left-color: #3b82f6;
    }

    .footer {
        padding: 1rem 2rem;
    }

    .logout {
        width: 100%;
        padding: 0.5rem;
        background: transparent;
        border: 1px solid #334155;
        color: #94a3b8;
        border-radius: 0.25rem;
        cursor: pointer;
    }

    .content {
        flex: 1;
        padding: 2rem 3rem;
    }

    header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 2rem;
    }

    h1 { font-size: 1.5rem; color: #1e293b; margin: 0; }
    h2 { font-size: 1.1rem; color: #1e293b; margin: 0; }

    .stats-grid {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 1.5rem;
        margin-bottom: 3rem;
    }

    .stat-card {
        background: white;
        padding: 1.5rem;
        border-radius: 0.5rem;
        box-shadow: 0 1px 3px rgba(0,0,0,0.1);
    }

    .stat-card .label { color: #64748b; font-size: 0.875rem; margin-bottom: 0.5rem; }
    .stat-card .value { font-size: 1.75rem; font-weight: 700; color: #0f172a; }
    .stat-card .status-ok { color: #10b981; }

    .table-section {
        background: white;
        border-radius: 0.5rem;
        box-shadow: 0 1px 3px rgba(0,0,0,0.1);
        overflow: hidden;
    }

    .section-header {
        padding: 1.5rem;
        border-bottom: 1px solid #f1f5f9;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    table {
        width: 100%;
        border-collapse: collapse;
        text-align: left;
    }

    th { padding: 1rem 1.5rem; background: #f8fafc; color: #64748b; font-size: 0.75rem; text-transform: uppercase; }
    td { padding: 1rem 1.5rem; border-bottom: 1px solid #f1f5f9; color: #334155; font-size: 0.875rem; }

    .badge {
        padding: 0.25rem 0.5rem;
        border-radius: 9999px;
        font-size: 0.75rem;
        font-weight: 600;
    }

    .badge.active { background: #dcfce7; color: #166534; }
    .badge.suspended { background: #fee2e2; color: #991b1b; }

    .btn-primary {
        background: #3b82f6;
        color: white;
        border: none;
        padding: 0.5rem 1rem;
        border-radius: 0.375rem;
        cursor: pointer;
        font-weight: 600;
    }

    .btn-text {
        background: transparent;
        border: none;
        color: #3b82f6;
        cursor: pointer;
        font-weight: 600;
    }

    code { background: #f1f5f9; padding: 0.2rem 0.4rem; border-radius: 0.25rem; font-family: monospace; }
</style>
