<script lang="ts">
	import { user, auth } from '$lib/stores/auth';
	import { page } from '$app/stores';

	let { 
		title = 'Dashboard', 
		subtitle = 'A good dashboard to display your statistics',
		children
	} = $props();

	// Sidebar state
	let isCollapsed = $state(false);
	// Dropdown states
	let showNotifDropdown = $state(false);
	let showProfileDropdown = $state(false);

	function toggleSidebar() {
		isCollapsed = !isCollapsed;
	}

	function toggleNotif(e: MouseEvent) {
		e.stopPropagation();
		showNotifDropdown = !showNotifDropdown;
		showProfileDropdown = false;
	}

	function toggleProfile(e: MouseEvent) {
		e.stopPropagation();
		showProfileDropdown = !showProfileDropdown;
		showNotifDropdown = false;
	}

	// Close dropdowns on outside click
	function closeAllDropdowns() {
		showNotifDropdown = false;
		showProfileDropdown = false;
	}

	const notifications = [
		{ id: 1, title: 'New Message', time: '5 min ago', icon: 'mail', color: 'bg-blue-100 text-blue-600' },
		{ id: 2, title: 'System Update', time: '1 hour ago', icon: 'settings', color: 'bg-green-100 text-green-600' },
		{ id: 3, title: 'New User Registered', time: '2 hours ago', icon: 'person_add', color: 'bg-orange-100 text-orange-600' }
	];

	const menuGroups = [
		{
			label: 'MAIN MENU',
			items: [
				{ name: 'Dashboard', href: '/dashboard', icon: 'grid_view' },
				{ name: 'Staff Management', href: '/owner/staff', icon: 'people' },
				{ name: 'Roles Management', href: '/owner/roles', icon: 'security' },
				{ name: 'Permissions', href: '/owner/permissions', icon: 'key' }
			]
		},
		{
			label: 'PLATFORM ADMIN',
			items: [
				{ name: 'Tenants (Companies)', href: '/platform/tenants', icon: 'business' },
				{ name: 'Global Users', href: '/platform/users', icon: 'manage_accounts' },
				{ name: 'Platform RBAC', href: '/owner/roles', icon: 'admin_panel_settings' }
			]
		},
		{
			label: 'COMPONENTS',
			items: [
				{ name: 'UI Components', href: '/dashboard/components', icon: 'category', subItems: true },
				{ name: 'Extra Components', href: '/dashboard/extra', icon: 'extension', subItems: true }
			]
		},
		{
			label: 'FORMS & TABLES',
			items: [
				{ name: 'Form Elements', href: '/dashboard/forms', icon: 'description', subItems: true },
				{ name: 'Form Layout', href: '/dashboard/layout', icon: 'view_quilt' },
				{ name: 'Table', href: '/dashboard/table', icon: 'table_chart' }
			]
		}
	];
</script>

<svelte:window onclick={closeAllDropdowns} />

<!-- Import Google Fonts and Material Icons -->
<svelte:head>
	<link href="https://fonts.googleapis.com/css2?family=Nunito:wght@300;400;600;700;800&display=swap" rel="stylesheet">
	<link href="https://fonts.googleapis.com/icon?family=Material+Icons+Outlined" rel="stylesheet">
</svelte:head>

<div class="app-container">
	<!-- Sidebar -->
	<aside class="sidebar" class:collapsed={isCollapsed}>
		<div class="sidebar-header">
			<div class="logo">{isCollapsed ? 'V' : 'Voler'}</div>
		</div>

		<nav class="sidebar-content">
			{#each menuGroups as group}
				<div class="menu-group-label">{isCollapsed ? '...' : group.label}</div>
				<ul class="menu-list">
					{#each group.items as item}
						<li class="menu-item" class:active={$page.url.pathname === item.href}>
							<a href={item.href} title={isCollapsed ? item.name : ''}>
								<span class="material-icons-outlined icon">{item.icon}</span>
								{#if !isCollapsed}
									<span class="name">{item.name}</span>
									{#if item.subItems}
										<span class="material-icons-outlined arrow">expand_more</span>
									{/if}
								{/if}
							</a>
						</li>
					{/each}
				</ul>
			{/each}
		</nav>
	</aside>

	<!-- Main Content Wrapper -->
	<div class="main-wrapper">
		<!-- Navbar -->
		<header class="navbar">
			<button class="menu-toggle" onclick={toggleSidebar}>
				<span class="material-icons-outlined">menu</span>
			</button>

			<div class="navbar-actions">
				<!-- Notifications -->
				<div class="dropdown-wrapper">
					<button class="action-btn" onclick={toggleNotif}>
						<span class="material-icons-outlined">notifications</span>
						<span class="badge"></span>
					</button>
					{#if showNotifDropdown}
						<div class="dropdown-menu notif-menu">
							<div class="dropdown-header">Notifications</div>
							<div class="dropdown-body">
								{#each notifications as notif}
									<div class="notif-item">
										<div class="notif-icon {notif.color}">
											<span class="material-icons-outlined">{notif.icon}</span>
										</div>
										<div class="notif-content">
											<div class="notif-title">{notif.title}</div>
											<div class="notif-time">{notif.time}</div>
										</div>
									</div>
								{/each}
							</div>
							<div class="dropdown-footer">View all notifications</div>
						</div>
					{/if}
				</div>

				<!-- Profile -->
				<div class="dropdown-wrapper">
					<button class="profile-toggle" onclick={toggleProfile}>
						<img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Saugi" alt="Avatar" class="avatar" />
						<div class="profile-info">
							<span class="greeting">Hi, {$user?.email?.split('@')[0] || 'Saugi'}</span>
						</div>
					</button>
					{#if showProfileDropdown}
						<div class="dropdown-menu profile-menu">
							<button class="dropdown-item">
								<span class="material-icons-outlined">person</span>
								Profile
							</button>
							<button class="dropdown-item">
								<span class="material-icons-outlined">lock_reset</span>
								Reset Password
							</button>
							<div class="dropdown-divider"></div>
							<button class="dropdown-item logout" onclick={auth.logout}>
								<span class="material-icons-outlined">logout</span>
								Logout
							</button>
						</div>
					{/if}
				</div>
			</div>
		</header>

		<!-- Content -->
		<main class="content">
			<header class="content-header">
				<h1>{title}</h1>
				<p class="subtitle">{subtitle}</p>
			</header>
			
			<div class="page-body">
				{@render children?.()}
			</div>
		</main>
	</div>
</div>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		font-family: 'Nunito', sans-serif;
		background-color: #f2f7ff;
		color: #25396f;
	}

	.app-container {
		display: flex;
		min-height: 100vh;
	}

	/* Sidebar Styles */
	.sidebar {
		width: 260px;
		background: white;
		height: 100vh;
		position: sticky;
		top: 0;
		display: flex;
		flex-direction: column;
		box-shadow: 0 0 20px rgba(0,0,0,0.05);
		z-index: 100;
		transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		overflow: hidden;
	}

	.sidebar.collapsed {
		width: 80px;
	}

	.sidebar-header {
		padding: 2rem;
		white-space: nowrap;
	}

	.logo {
		font-size: 2rem;
		font-weight: 800;
		color: #435ebe;
		letter-spacing: -1px;
		transition: all 0.3s;
	}

	.sidebar.collapsed .logo {
		text-align: center;
	}

	.sidebar-content {
		flex: 1;
		overflow-y: auto;
		overflow-x: hidden;
		padding: 0 1rem 2rem 1rem;
	}

	.menu-group-label {
		font-size: 0.75rem;
		font-weight: 700;
		color: #a0aec0;
		padding: 1.5rem 1rem 0.5rem 1rem;
		letter-spacing: 0.5px;
		white-space: nowrap;
		text-overflow: ellipsis;
		overflow: hidden;
	}

	.sidebar.collapsed .menu-group-label {
		text-align: center;
		padding-left: 0;
		padding-right: 0;
	}

	.menu-list {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.menu-item {
		margin-bottom: 0.25rem;
	}

	.menu-item a {
		display: flex;
		align-items: center;
		padding: 0.75rem 1rem;
		text-decoration: none;
		color: #25396f;
		font-weight: 600;
		border-radius: 0.5rem;
		transition: all 0.2s;
		white-space: nowrap;
	}

	.sidebar.collapsed .menu-item a {
		justify-content: center;
		padding: 0.75rem 0;
	}

	.menu-item a:hover {
		background-color: #f2f7ff;
	}

	.menu-item.active a {
		background-color: #ebf3ff;
		color: #435ebe;
	}

	.menu-item .icon {
		margin-right: 1rem;
		font-size: 1.25rem;
		color: #7c8db5;
		transition: margin 0.3s;
	}

	.sidebar.collapsed .icon {
		margin-right: 0;
	}

	.menu-item.active .icon {
		color: #435ebe;
	}

	.menu-item .name {
		flex: 1;
		opacity: 1;
		transition: opacity 0.2s;
	}

	.menu-item .arrow {
		font-size: 1.25rem;
		color: #a0aec0;
	}

	/* Main Wrapper Styles */
	.main-wrapper {
		flex: 1;
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	/* Navbar Styles */
	.navbar {
		height: 70px;
		background: transparent;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0 2rem;
	}

	.menu-toggle {
		background: none;
		border: none;
		color: #25396f;
		cursor: pointer;
		padding: 0.5rem;
		z-index: 101;
	}

	.navbar-actions {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.action-btn {
		background: none;
		border: none;
		color: #7c8db5;
		cursor: pointer;
		position: relative;
		padding: 0.5rem;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 50%;
		transition: background 0.2s;
	}

	.action-btn:hover {
		background: #f1f5f9;
	}

	.badge {
		position: absolute;
		top: 6px;
		right: 6px;
		width: 8px;
		height: 8px;
		background: #ef4444;
		border: 2px solid white;
		border-radius: 50%;
	}

	.dropdown-wrapper {
		position: relative;
	}

	.dropdown-menu {
		position: absolute;
		top: calc(100% + 10px);
		right: 0;
		background: white;
		border-radius: 0.75rem;
		box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 8px 10px -6px rgba(0, 0, 0, 0.1);
		min-width: 240px;
		z-index: 1000;
		padding: 0.5rem 0;
		border: 1px solid #f1f5f9;
		animation: slideIn 0.2s ease-out;
	}

	@keyframes slideIn {
		from { opacity: 0; transform: translateY(-10px); }
		to { opacity: 1; transform: translateY(0); }
	}

	.dropdown-header {
		padding: 0.75rem 1.25rem;
		font-weight: 800;
		font-size: 0.85rem;
		color: #25396f;
		border-bottom: 1px solid #f1f5f9;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.notif-menu { width: 320px; }

	.notif-item {
		display: flex;
		gap: 1rem;
		padding: 0.75rem 1.25rem;
		cursor: pointer;
		transition: background 0.2s;
	}

	.notif-item:hover { background: #f8fafc; }

	.notif-icon {
		width: 36px;
		height: 36px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.notif-icon .material-icons-outlined { font-size: 1.25rem; }

	.bg-blue-100 { background: #ebf3ff; color: #435ebe; }
	.bg-green-100 { background: #dcfce7; color: #16a34a; }
	.bg-orange-100 { background: #ffedd5; color: #ea580c; }

	.notif-title { font-weight: 700; font-size: 0.9rem; color: #25396f; }
	.notif-time { font-size: 0.75rem; color: #7c8db5; }

	.dropdown-footer {
		padding: 0.75rem;
		text-align: center;
		font-size: 0.85rem;
		font-weight: 700;
		color: #435ebe;
		cursor: pointer;
		border-top: 1px solid #f1f5f9;
	}

	.profile-toggle {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem 1rem;
		background: none;
		border: none;
		border-left: 1px solid #e2e8f0;
		cursor: pointer;
		transition: background 0.2s;
	}

	.profile-toggle:hover { background: #f8fafc; }

	.avatar {
		width: 38px;
		height: 38px;
		border-radius: 50%;
		background: #eee;
	}

	.greeting {
		font-weight: 700;
		font-size: 0.9rem;
		color: #25396f;
	}

	.profile-menu { min-width: 200px; }

	.dropdown-item {
		width: 100%;
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1.25rem;
		border: none;
		background: none;
		font-family: inherit;
		font-size: 0.9rem;
		font-weight: 600;
		color: #435ebe;
		cursor: pointer;
		transition: background 0.2s;
		text-align: left;
	}

	.dropdown-item:hover { background: #f8fafc; }
	.dropdown-item.logout { color: #ef4444; }
	.dropdown-divider { height: 1px; background: #f1f5f9; margin: 0.5rem 0; }

	/* Content Styles */
	.content {
		padding: 0 2rem 2rem 2rem;
	}

	.content-header {
		margin-bottom: 2rem;
	}

	.content-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		margin: 0 0 0.25rem 0;
	}

	.subtitle {
		color: #7c8db5;
		margin: 0;
		font-weight: 600;
	}

	.page-body {
		/* Grid layout will be in +page.svelte */
	}

	/* Responsive */
	@media (max-width: 992px) {
		.sidebar {
			display: none;
		}
	}
</style>
