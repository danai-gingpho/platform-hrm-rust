<script lang="ts">
	import type { Permission } from '$lib/types';
	import { onMount } from 'svelte';

	let permissions = $state<Permission[]>([]);
	let isLoading = $state(true);
	let searchQuery = $state('');
	let showModal = $state(false);
	let editingPerm = $state<Permission | null>(null);

	// Mock Data
	const mockPermissions: Permission[] = [
		{ id: '1', name: 'View Dashboard', code: 'dash:view', description: 'Allows viewing general statistics and overview' },
		{ id: '2', name: 'Manage Staff', code: 'staff:manage', description: 'Can create, update, and delete employees' },
		{ id: '3', name: 'Process Payroll', code: 'payroll:process', description: 'Full access to payroll processing and items' },
		{ id: '4', name: 'System Config', code: 'sys:config', description: 'Access to global system settings and logs' },
		{ id: '5', name: 'View Reports', code: 'report:view', description: 'Can view and export analytical reports' }
	];

	onMount(() => {
		setTimeout(() => {
			permissions = mockPermissions;
			isLoading = false;
		}, 500);
	});

	let filteredPerms = $derived(
		permissions.filter(p => 
			p.name.toLowerCase().includes(searchQuery.toLowerCase()) || 
			p.code.toLowerCase().includes(searchQuery.toLowerCase())
		)
	);

	function openAddModal() {
		editingPerm = null;
		showModal = true;
	}

	function openEditModal(perm: Permission) {
		editingPerm = { ...perm };
		showModal = true;
	}

	function deletePerm(id: string) {
		if (confirm('Are you sure? Deleting a permission may affect roles that use it.')) {
			permissions = permissions.filter(p => p.id !== id);
		}
	}

	function handleSave(e: SubmitEvent) {
		e.preventDefault();
		const formData = new FormData(e.target as HTMLFormElement);
		const data = Object.fromEntries(formData) as any;

		if (editingPerm) {
			permissions = permissions.map(p => p.id === editingPerm!.id ? { ...p, ...data } : p);
		} else {
			const newPerm: Permission = {
				id: Math.random().toString(36).substr(2, 9),
				...data
			};
			permissions = [newPerm, ...permissions];
		}
		showModal = false;
	}
</script>

<div class="page-container">
	<div class="table-header-actions">
		<div class="title-area">
			<h2>Permissions Directory</h2>
			<p>Define and manage system-wide access tokens</p>
		</div>
		<button class="btn btn-primary" onclick={openAddModal}>
			<span class="material-icons-outlined">key</span>
			Create Permission
		</button>
	</div>

	<div class="search-bar-row">
		<div class="search-box">
			<span class="material-icons-outlined">search</span>
			<input type="text" placeholder="Search by name or code..." bind:value={searchQuery} />
		</div>
	</div>

	<div class="card table-card">
		{#if isLoading}
			<div class="loading-state">Syncing permissions...</div>
		{:else}
			<table class="data-table">
				<thead>
					<tr>
						<th>Permission Name</th>
						<th>Code Token</th>
						<th>Description</th>
						<th class="text-right">Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each filteredPerms as perm}
						<tr>
							<td>
								<div class="perm-name-cell">
									<span class="material-icons-outlined">api</span>
									<span class="font-bold">{perm.name}</span>
								</div>
							</td>
							<td><code class="code-badge">{perm.code}</code></td>
							<td class="desc-cell">{perm.description}</td>
							<td class="text-right">
								<button class="icon-btn edit" onclick={() => openEditModal(perm)}>
									<span class="material-icons-outlined">edit</span>
								</button>
								<button class="icon-btn delete" onclick={() => deletePerm(perm.id)}>
									<span class="material-icons-outlined">delete</span>
								</button>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		{/if}
	</div>
</div>

{#if showModal}
	<div class="modal-backdrop">
		<div class="modal">
			<div class="modal-header">
				<h3>{editingPerm ? 'Edit Permission' : 'New Permission'}</h3>
				<button class="close-btn" onclick={() => showModal = false}>&times;</button>
			</div>
			<form onsubmit={handleSave}>
				<div class="modal-body">
					<div class="form-group">
						<label for="name">Display Name</label>
						<input type="text" id="name" name="name" value={editingPerm?.name || ''} required placeholder="e.g. Edit Salaries" />
					</div>
					<div class="form-group">
						<label for="code">Permission Code (Token)</label>
						<input type="text" id="code" name="code" value={editingPerm?.code || ''} required placeholder="e.g. payroll:edit" />
						<small>Unique string used for programmatic checks</small>
					</div>
					<div class="form-group">
						<label for="description">Detailed Description</label>
						<textarea id="description" name="description" rows="3" required>{editingPerm?.description || ''}</textarea>
					</div>
				</div>
				<div class="modal-footer">
					<button type="button" class="btn btn-light" onclick={() => showModal = false}>Cancel</button>
					<button type="submit" class="btn btn-primary">Save Permission</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	.page-container { animation: fadeIn 0.3s ease-out; }
	@keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }

	.table-header-actions { display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 2rem; }
	.title-area h2 { margin: 0; color: #25396f; font-weight: 800; }
	.title-area p { margin: 0.25rem 0 0 0; color: #7c8db5; font-weight: 600; font-size: 0.95rem; }

	.search-bar-row { margin-bottom: 1.5rem; }
	.search-box { background: white; border-radius: 0.75rem; display: flex; align-items: center; padding: 0 1rem; border: 1px solid #e2e8f0; max-width: 500px; }
	.search-box input { border: none; padding: 0.85rem; width: 100%; outline: none; font-family: inherit; font-weight: 600; }
	.search-box .material-icons-outlined { color: #7c8db5; }

	.btn { display: flex; align-items: center; gap: 0.5rem; padding: 0.85rem 1.5rem; border-radius: 0.75rem; font-weight: 700; cursor: pointer; border: none; transition: all 0.2s; }
	.btn-primary { background: #435ebe; color: white; box-shadow: 0 4px 12px rgba(67, 94, 190, 0.25); }
	.btn-primary:hover { background: #364b9a; transform: translateY(-1px); }
	.btn-light { background: #f1f5f9; color: #25396f; }

	.card { background: white; border-radius: 1rem; box-shadow: 0 0 25px rgba(0,0,0,0.03); overflow: hidden; border: 1px solid #f1f5f9; }
	.data-table { width: 100%; border-collapse: collapse; text-align: left; }
	.data-table th { padding: 1.25rem 1.5rem; background: #f8fafc; color: #7c8db5; font-size: 0.8rem; text-transform: uppercase; letter-spacing: 1px; }
	.data-table td { padding: 1.25rem 1.5rem; border-bottom: 1px solid #f1f5f9; color: #25396f; font-weight: 600; font-size: 0.95rem; }

	.perm-name-cell { display: flex; align-items: center; gap: 0.75rem; color: #435ebe; }
	.perm-name-cell .material-icons-outlined { font-size: 1.1rem; opacity: 0.7; }

	.code-badge { background: #f1f5f9; color: #435ebe; padding: 0.25rem 0.6rem; border-radius: 6px; font-family: 'Monaco', 'Consolas', monospace; font-size: 0.85rem; }
	.desc-cell { color: #7c8db5; font-weight: 500; font-size: 0.9rem; max-width: 400px; }

	.icon-btn { background: none; border: none; cursor: pointer; padding: 0.5rem; border-radius: 8px; transition: all 0.2s; }
	.icon-btn.edit { color: #435ebe; }
	.icon-btn.delete { color: #ef4444; }
	.icon-btn:hover { background: #f1f5f9; }

	.text-right { text-align: right; }
	.loading-state { padding: 5rem; text-align: center; color: #7c8db5; font-weight: 600; font-size: 1.1rem; }

	/* Modal Styles (Consistent with Staff/Roles) */
	.modal-backdrop { position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(37, 57, 111, 0.4); display: flex; align-items: center; justify-content: center; z-index: 2000; backdrop-filter: blur(6px); }
	.modal { background: white; border-radius: 1.25rem; width: 100%; max-width: 500px; box-shadow: 0 25px 50px -12px rgba(0,0,0,0.2); animation: modalIn 0.3s cubic-bezier(0.34, 1.56, 0.64, 1); }
	@keyframes modalIn { from { opacity: 0; transform: scale(0.95) translateY(20px); } to { opacity: 1; transform: scale(1) translateY(0); } }
	
	.modal-header { padding: 1.5rem; border-bottom: 1px solid #f1f5f9; display: flex; justify-content: space-between; align-items: center; }
	.modal-header h3 { margin: 0; color: #25396f; font-weight: 800; }
	.close-btn { background: none; border: none; font-size: 1.75rem; cursor: pointer; color: #7c8db5; line-height: 1; }
	
	.modal-body { padding: 1.5rem; }
	.modal-footer { padding: 1.5rem; border-top: 1px solid #f1f5f9; display: flex; justify-content: flex-end; gap: 1rem; }

	.form-group { margin-bottom: 1.5rem; }
	.form-group small { display: block; margin-top: 0.5rem; color: #a0aec0; font-weight: 600; }
	label { display: block; font-weight: 700; font-size: 0.95rem; color: #25396f; margin-bottom: 0.6rem; }
	input, textarea { width: 100%; padding: 0.85rem; border: 2px solid #f1f5f9; border-radius: 0.75rem; font-family: inherit; font-weight: 600; transition: border-color 0.2s; }
	input:focus, textarea:focus { outline: none; border-color: #435ebe; }
	textarea { resize: none; }
</style>
