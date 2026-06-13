<script lang="ts">
	import { user, auth } from '$lib/stores/auth';
	import { page } from '$app/stores';
	import SystemStatus from './SystemStatus.svelte';

	export let title = 'Platform HRM';

	const navItems = [
		{ name: 'Dashboard', href: '/dashboard', icon: 'dashboard' },
		{ name: 'Employees', href: '/dashboard/employees', icon: 'group' },
		{ name: 'Payroll', href: '/dashboard/payroll', icon: 'payments' },
		{ name: 'Settings', href: '/dashboard/settings', icon: 'settings' }
	];

	$: currentPath = $page.url.pathname;
</script>

<div class="flex h-screen bg-gray-100">
	<!-- Sidebar -->
	<aside class="w-64 bg-slate-900 text-white flex flex-col">
		<div class="p-6 text-xl font-bold border-b border-slate-800">HRM Admin</div>

		<nav class="flex-1 mt-6 px-4 space-y-1">
			{#each navItems as item}
				<a
					href={item.href}
					class="flex items-center px-4 py-3 rounded-lg transition-colors {currentPath === item.href
						? 'bg-blue-600 text-white'
						: 'text-slate-400 hover:bg-slate-800 hover:text-white'}"
				>
					<span class="capitalize">{item.name}</span>
				</a>
			{/each}
		</nav>

		<div class="p-4 border-t border-slate-800">
			<div class="text-sm text-slate-400">Logged in as:</div>
			<div class="text-sm font-medium truncate">{$user?.email || 'User'}</div>
			<button
				on:click={auth.logout}
				class="mt-4 w-full text-left px-4 py-2 text-sm text-red-400 hover:bg-slate-800 rounded transition-colors"
			>
				Sign Out
			</button>
		</div>
	</aside>

	<!-- Main Content -->
	<main class="flex-1 flex flex-col overflow-hidden">
		<!-- Top Navbar -->
		<header class="h-16 bg-white border-b flex items-center justify-between px-8">
			<h2 class="text-lg font-semibold text-gray-800">{title}</h2>
			<div class="flex items-center space-x-4">
				<div class="w-8 h-8 rounded-full bg-blue-100 text-blue-600 flex items-center justify-center font-bold">
					{$user?.email?.[0].toUpperCase() || 'U'}
				</div>
			</div>
		</header>

		<!-- Page Content -->
		<section class="flex-1 overflow-y-auto p-8">
			<slot />
		</section>
	</main>
</div>
