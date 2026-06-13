<script lang="ts">
	import type { Role, Permission } from '$lib/types';
	
	let searchQuery = $state('');
	let showModal = $state(false);
	let editingRole = $state<Role | null>(null);

	const permissions: Permission[] = [
		{ id: '1', name: 'View Dashboard', code: 'dash:view', description: 'Allows viewing general statistics' },
		{ id: '2', name: 'Manage Staff', code: 'staff:manage', description: 'Can create, update, and delete employees' },
		{ id: '3', name: 'Process Payroll', code: 'payroll:process', description: 'Full access to payroll module' },
		{ id: '4', name: 'System Config', code: 'sys:config', description: 'Access to global system settings' }
	];

	let roles = $state<Role[]>([
		{ id: '1', name: 'Administrator', description: 'Full access to all modules', permissions: ['1', '2', '3', '4'] },
		{ id: '2', name: 'HR Manager', description: 'Management of staff and leave', permissions: ['1', '2'] },
		{ id: '3', name: 'Payroll Specialist', description: 'Specialized in financial items', permissions: ['1', '3'] }
	]);

	let selectedPermissions = $state<string[]>([]);

	function openAddRole() {
		editingRole = null;
		selectedPermissions = [];
		showModal = true;
	}

	function openEditRole(role: Role) {
		editingRole = { ...role };
		selectedPermissions = [...role.permissions];
		showModal = true;
	}

	function togglePermission(id: string) {
		if (selectedPermissions.includes(id)) {
			selectedPermissions = selectedPermissions.filter(p => p !== id);
		} else {
			selectedPermissions = [...selectedPermissions, id];
		}
	}

	function handleSave(e: SubmitEvent) {
		e.preventDefault();
		const formData = new FormData(e.target as HTMLFormElement);
		const name = formData.get('name') as string;
		const description = formData.get('description') as string;

		if (editingRole) {
			roles = roles.map(r => r.id === editingRole!.id ? { ...r, name, description, permissions: selectedPermissions } : r);
		} else {
			const newRole: Role = {
				id: Math.random().toString(36).substr(2, 9),
				name,
				description,
				permissions: selectedPermissions
			};
			roles = [...roles, newRole];
		}
		showModal = false;
	}
</script>

<div class="page-container">
	<div class="table-header-actions">
		<h2>Role Based Access Control</h2>
		<button class="btn btn-primary" onclick={openAddRole}>
			<span class="material-icons-outlined">security</span>
			Create New Role
		</button>
	</div>

	<div class="roles-grid">
		{#each roles as role}
			<div class="card role-card">
				<div class="role-icon">
					<span class="material-icons-outlined">verified_user</span>
				</div>
				<div class="role-info">
					<h3>{role.name}</h3>
					<p>{role.description}</p>
					<div class="permission-count">
						<span class="material-icons-outlined">key</span>
						{role.permissions.length} Permissions active
					</div>
				</div>
				<div class="role-actions">
					<button class="btn-text" onclick={() => openEditRole(role)}>Manage Permissions</button>
					<button class="icon-btn delete"><span class="material-icons-outlined">delete</span></button>
				</div>
			</div>
		{/each}
	</div>
</div>

{#if showModal}
	<div class="modal-backdrop">
		<div class="modal">
			<div class="modal-header">
				<h3>{editingRole ? 'Edit Role' : 'Create New Role'}</h3>
				<button class="close-btn" onclick={() => showModal = false}>&times;</button>
			</div>
			<form onsubmit={handleSave}>
				<div class="modal-body">
					<div class="form-group">
						<label for="name">Role Name</label>
						<input type="text" id="name" name="name" value={editingRole?.name || ''} required placeholder="e.g. Content Manager" />
					</div>
					<div class="form-group">
						<label for="description">Description</label>
						<input type="text" id="description" name="description" value={editingRole?.description || ''} placeholder="Briefly describe what this role does" />
					</div>
					
					<div class="permissions-section">
						<label>Assign Permissions</label>
						<div class="permissions-list">
							{#each permissions as perm}
								<label class="perm-item" class:selected={selectedPermissions.includes(perm.id)}>
									<input 
										type="checkbox" 
										checked={selectedPermissions.includes(perm.id)} 
										onchange={() => togglePermission(perm.id)} 
									/>
									<div class="perm-info">
										<span class="perm-name">{perm.name}</span>
										<span class="perm-code">{perm.code}</span>
									</div>
								</label>
							{/each}
						</div>
					</div>
				</div>
				<div class="modal-footer">
					<button type="button" class="btn btn-light" onclick={() => showModal = false}>Cancel</button>
					<button type="submit" class="btn btn-primary">Save Role Configuration</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	.page-container { animation: fadeIn 0.3s ease-out; }
	@keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }

	.table-header-actions {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.table-header-actions h2 { margin: 0; font-size: 1.5rem; font-weight: 800; color: #25396f; }

	.roles-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 1.5rem;
	}

	.role-card {
		background: white;
		border-radius: 1rem;
		padding: 1.5rem;
		box-shadow: 0 0 20px rgba(0,0,0,0.05);
		display: flex;
		flex-direction: column;
		gap: 1rem;
		border: 2px solid transparent;
		transition: all 0.2s;
	}

	.role-card:hover { border-color: #435ebe; transform: translateY(-5px); }

	.role-icon {
		width: 48px;
		height: 48px;
		background: #ebf3ff;
		color: #435ebe;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.role-info h3 { margin: 0; color: #25396f; font-size: 1.25rem; }
	.role-info p { color: #7c8db5; font-size: 0.9rem; margin: 0.5rem 0; font-weight: 600; }

	.permission-count {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.85rem;
		font-weight: 700;
		color: #435ebe;
		margin-top: 1rem;
	}

	.permission-count .material-icons-outlined { font-size: 1rem; }

	.role-actions {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid #f1f5f9;
	}

	.btn-text {
		background: none;
		border: none;
		color: #435ebe;
		font-weight: 800;
		font-size: 0.9rem;
		cursor: pointer;
		padding: 0;
	}

	/* Reuse generic styles from staff page for modal and buttons */
	.btn { display: flex; align-items: center; gap: 0.5rem; padding: 0.75rem 1.25rem; border-radius: 0.5rem; font-weight: 700; cursor: pointer; border: none; }
	.btn-primary { background: #435ebe; color: white; }
	.btn-light { background: #f1f5f9; color: #25396f; }
	.icon-btn.delete { color: #ef4444; background: none; border: none; cursor: pointer; }

	/* Permissions Checklist */
	.permissions-section label { margin-bottom: 1rem; display: block; }
	.permissions-list {
		display: grid;
		grid-template-columns: 1fr;
		gap: 0.75rem;
		max-height: 250px;
		overflow-y: auto;
		padding-right: 0.5rem;
	}

	.perm-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem;
		border: 2px solid #f1f5f9;
		border-radius: 0.75rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.perm-item.selected { border-color: #435ebe; background: #f0f3ff; }
	.perm-item input { width: auto; }

	.perm-info { display: flex; flex-direction: column; }
	.perm-name { font-weight: 700; font-size: 0.9rem; color: #25396f; }
	.perm-code { font-size: 0.75rem; color: #7c8db5; font-weight: 600; }

	/* Modal generic styles */
	.modal-backdrop { position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 2000; backdrop-filter: blur(4px); }
	.modal { background: white; border-radius: 1rem; width: 100%; max-width: 550px; box-shadow: 0 20px 25px -5px rgba(0,0,0,0.1); }
	.modal-header { padding: 1.5rem; border-bottom: 1px solid #f1f5f9; display: flex; justify-content: space-between; align-items: center; }
	.close-btn { background: none; border: none; font-size: 1.5rem; cursor: pointer; color: #7c8db5; }
	.modal-body { padding: 1.5rem; }
	.modal-footer { padding: 1.25rem 1.5rem; border-top: 1px solid #f1f5f9; display: flex; justify-content: flex-end; gap: 1rem; }
	.form-group { margin-bottom: 1.25rem; }
	input { width: 100%; padding: 0.75rem; border: 2px solid #f1f5f9; border-radius: 0.5rem; font-family: inherit; font-weight: 600; }
</style>
