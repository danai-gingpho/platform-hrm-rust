<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { isAuthenticated } from '$lib/stores/auth';

	onMount(() => {
		// Check authentication status and redirect accordingly
		if ($isAuthenticated) {
			goto('/dashboard');
		} else {
			goto('/login');
		}
	});
</script>

<div class="flex items-center justify-center min-h-screen bg-gray-50">
	<div class="text-center">
		<div class="inline-block w-8 h-8 border-4 border-blue-600 border-t-transparent rounded-full animate-spin mb-4"></div>
		<p class="text-gray-600 font-medium">Loading Platform HRM...</p>
	</div>
</div>

<style>
	/* Basic spinner animation if Tailwind is not fully set up */
	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}
	.animate-spin {
		animation: spin 1s linear infinite;
	}
</style>
