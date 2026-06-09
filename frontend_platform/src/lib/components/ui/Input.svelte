<script lang="ts">
	export let label = '';
	export let name = '';
	export let type = 'text';
	export let placeholder = '';
	export let value = '';
	export let error = '';
	export let required = false;
</script>

<div class="w-full space-y-1.5">
	{#if label}
		<label for={name} class="block text-sm font-medium text-gray-700">
			{label} {#if required}<span class="text-red-500">*</span>{/if}
		</label>
	{/if}
	
	<input
		{id: name}
		{name}
		{type}
		{placeholder}
		{required}
		bind:value
		class="block w-full px-4 py-2 bg-white border rounded-lg text-gray-900 focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all {error ? 'border-red-500 bg-red-50' : 'border-gray-300'}"
	/>

	{#if error}
		<p class="text-xs text-red-600 mt-1">{error}</p>
	{/if}
</div>
