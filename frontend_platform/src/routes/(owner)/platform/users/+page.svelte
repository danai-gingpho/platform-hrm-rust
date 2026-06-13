<script lang="ts">
	import type { PlatformUser } from '$lib/types/platform';
	import { onMount } from 'svelte';

	let users = $state<PlatformUser[]>([]);
	let showModal = $state(false);
	let editingUser = $state<PlatformUser | null>(null);

	const mockUsers: PlatformUser[] = [
		{ id: '1', email: 'admin@platform.com', tenant_id: '0', tenant_name: 'System', global_role: 'platform_admin', status: 'active' },
		{ id: '2', email: 'ceo@acme.com', tenant_id: '1', tenant_name: 'Acme Corp', global_role: 'tenant_owner', status: 'active' },
		{ id: '3', email: 'hr@global.com', tenant_id: '2', tenant_name: 'Global Tech', global_role: 'tenant_user', status: 'active' }
	];

	onMount(() => { users = mockUsers; });

	function handleSave(e: SubmitEvent) {
		e.preventDefault();
		showModal = false;
	}
</script>

<div class="page-container">
	<div class="header-section">
		<div>
			<h1>Platform Users</h1>
			<p>Global user directory across all tenants</p>
		</div>
		<button class="btn btn-primary" onclick={() => { editingUser = null; showModal = true; }}>
			<span class="material-icons-outlined">person_add</span>
			Create Global User
		</button>
	</div>

	<div class="card">
		<table class="data-table">
			<thead>
				<tr>
					<th>User Email</th>
					<th>Assigned Tenant</th>
					<th>Global Role</th>
					<th>Status</th>
					<th class="text-right">Actions</th>
				</tr>
			</thead>
			<tbody>
				{#each users as user}
					<tr>
						<td><span class="font-bold">{user.email}</span></td>
						<td><span class="tenant-tag">{user.tenant_name}</span></td>
						<td><code class="role-code">{user.global_role}</code></td>
						<td><span class="status-pill {user.status}">{user.status}</span></td>
						<td class="text-right">
							<button class="icon-btn" onclick={() => { editingUser = user; showModal = true; }}>
								<span class="material-icons-outlined">edit</span>
							</button>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>

{#if showModal}
	<div class="modal-backdrop">
		<div class="modal">
			<div class="modal-header">
				<h3>{editingUser ? 'Edit Global User' : 'Create Global User'}</h3>
				<button class="close-btn" onclick={() => showModal = false}>&times;</button>
			</div>
			<form onsubmit={handleSave}>
				<div class="modal-body">
					<div class="form-group">
						<label for="email">Email Address</label>
						<input type="email" name="email" value={editingUser?.email || ''} required />
					</div>
					<div class="form-group">
						<label for="tenant">Assign to Tenant</label>
						<select name="tenant">
							<option value="0">System (Platform Admin)</option>
							<option value="1">Acme Corp</option>
							<option value="2">Global Tech</option>
						</select>
					</div>
					<div class="form-group">
						<label for="role">Global Role</label>
						<select name="role" value={editingUser?.global_role || 'tenant_user'}>
							<option value="platform_admin">Platform Admin</option>
							<option value="tenant_owner">Tenant Owner</option>
							<option value="tenant_user">Tenant User</option>
						</select>
					</div>
				</div>
				<div class="modal-footer">
					<button type="button" class="btn btn-light" onclick={() => showModal = false}>Cancel</button>
					<button type="submit" class="btn btn-primary">Save User</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	.header-section { display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem; }
	.header-section h1 { margin: 0; color: #25396f; font-weight: 800; }
	.card { background: white; border-radius: 1rem; box-shadow: 0 0 20px rgba(0,0,0,0.05); overflow: hidden; }
	.data-table { width: 100%; border-collapse: collapse; text-align: left; }
	.data-table th { padding: 1.25rem; background: #f8fafc; color: #7c8db5; font-size: 0.8rem; text-transform: uppercase; }
	.data-table td { padding: 1.25rem; border-bottom: 1px solid #f1f5f9; color: #25396f; font-weight: 600; }
	
	.tenant-tag { background: #f1f5f9; padding: 0.2rem 0.6rem; border-radius: 6px; font-size: 0.85rem; }
	.role-code { color: #435ebe; font-weight: 800; }
	
	.btn { display: flex; align-items: center; gap: 0.5rem; padding: 0.85rem 1.5rem; border-radius: 0.75rem; font-weight: 700; cursor: pointer; border: none; }
	.btn-primary { background: #435ebe; color: white; }
	.btn-light { background: #f1f5f9; color: #25396f; }
	.icon-btn { background: none; border: none; cursor: pointer; color: #7c8db5; padding: 0.5rem; }
	.status-pill.active { background: #dcfce7; color: #16a34a; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.75rem; }
	.text-right { text-align: right; }

	.modal-backdrop { position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.4); display: flex; align-items: center; justify-content: center; z-index: 2000; backdrop-filter: blur(4px); }
	.modal { background: white; border-radius: 1.25rem; width: 100%; max-width: 500px; }
	.modal-header { padding: 1.5rem; border-bottom: 1px solid #f1f5f9; display: flex; justify-content: space-between; }
	.modal-body { padding: 1.5rem; }
	.modal-footer { padding: 1.5rem; border-top: 1px solid #f1f5f9; display: flex; justify-content: flex-end; gap: 1rem; }
	.form-group { margin-bottom: 1.25rem; }
	label { display: block; font-weight: 700; margin-bottom: 0.5rem; color: #25396f; }
	input, select { width: 100%; padding: 0.75rem; border: 2px solid #f1f5f9; border-radius: 0.5rem; font-family: inherit; font-weight: 600; }
	.close-btn { background: none; border: none; font-size: 1.5rem; cursor: pointer; }
</style>
