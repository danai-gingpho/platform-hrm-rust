<script lang="ts">
	import type { Staff } from '$lib/types';
	import { onMount } from 'svelte';

	let staffList = $state<Staff[]>([]);
	let isLoading = $state(true);
	let searchQuery = $state('');
	let showModal = $state(false);
	let editingStaff = $state<Staff | null>(null);

	// Mock Data
	const mockStaff: Staff[] = [
		{ id: '1', name: 'John Doe', email: 'john@company.com', role: 'Administrator', status: 'active', joinedDate: '2024-01-15' },
		{ id: '2', name: 'Jane Smith', email: 'jane@company.com', role: 'HR Manager', status: 'active', joinedDate: '2024-02-10' },
		{ id: '3', name: 'Bob Wilson', email: 'bob@company.com', role: 'Developer', status: 'inactive', joinedDate: '2023-11-05' },
		{ id: '4', name: 'Alice Brown', email: 'alice@company.com', role: 'Designer', status: 'pending', joinedDate: '2024-05-20' }
	];

	onMount(() => {
		setTimeout(() => {
			staffList = mockStaff;
			isLoading = false;
		}, 800);
	});

	let filteredStaff = $derived(
		staffList.filter(s => 
			s.name.toLowerCase().includes(searchQuery.toLowerCase()) || 
			s.email.toLowerCase().includes(searchQuery.toLowerCase())
		)
	);

	function openAddModal() {
		editingStaff = null;
		showModal = true;
	}

	function openEditModal(staff: Staff) {
		editingStaff = { ...staff };
		showModal = true;
	}

	function deleteStaff(id: string) {
		if (confirm('Are you sure you want to delete this staff member?')) {
			staffList = staffList.filter(s => s.id !== id);
		}
	}

	function handleSave(e: SubmitEvent) {
		e.preventDefault();
		const formData = new FormData(e.target as HTMLFormElement);
		const data = Object.fromEntries(formData) as any;

		if (editingStaff) {
			staffList = staffList.map(s => s.id === editingStaff!.id ? { ...s, ...data } : s);
		} else {
			const newStaff: Staff = {
				id: Math.random().toString(36).substr(2, 9),
				joinedDate: new Date().toISOString().split('T')[0],
				status: 'active',
				...data
			};
			staffList = [newStaff, ...staffList];
		}
		showModal = false;
	}
</script>

<div class="page-container">
	<div class="table-header-actions">
		<div class="search-box">
			<span class="material-icons-outlined">search</span>
			<input type="text" placeholder="Search staff..." bind:value={searchQuery} />
		</div>
		<button class="btn btn-primary" onclick={openAddModal}>
			<span class="material-icons-outlined">add</span>
			Add Staff
		</button>
	</div>

	<div class="card table-card">
		{#if isLoading}
			<div class="loading-state">Loading staff directory...</div>
		{:else}
			<table class="data-table">
				<thead>
					<tr>
						<th>Name</th>
						<th>Email</th>
						<th>Role</th>
						<th>Status</th>
						<th>Joined Date</th>
						<th class="text-right">Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each filteredStaff as staff}
						<tr>
							<td>
								<div class="user-info">
									<div class="avatar-small">{staff.name[0]}</div>
									<span class="font-bold">{staff.name}</span>
								</div>
							</td>
							<td>{staff.email}</td>
							<td><span class="badge-outline">{staff.role}</span></td>
							<td>
								<span class="status-pill {staff.status}">
									{staff.status}
								</span>
							</td>
							<td>{staff.joinedDate}</td>
							<td class="text-right">
								<button class="icon-btn edit" onclick={() => openEditModal(staff)}>
									<span class="material-icons-outlined">edit</span>
								</button>
								<button class="icon-btn delete" onclick={() => deleteStaff(staff.id)}>
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
				<h3>{editingStaff ? 'Edit Staff' : 'Add New Staff'}</h3>
				<button class="close-btn" onclick={() => showModal = false}>&times;</button>
			</div>
			<form onsubmit={handleSave}>
				<div class="modal-body">
					<div class="form-group">
						<label for="name">Full Name</label>
						<input type="text" id="name" name="name" value={editingStaff?.name || ''} required />
					</div>
					<div class="form-group">
						<label for="email">Email Address</label>
						<input type="email" id="email" name="email" value={editingStaff?.email || ''} required />
					</div>
					<div class="form-group">
						<label for="role">Department / Role</label>
						<select id="role" name="role" value={editingStaff?.role || 'Developer'}>
							<option>Administrator</option>
							<option>HR Manager</option>
							<option>Developer</option>
							<option>Designer</option>
							<option>Sales</option>
						</select>
					</div>
				</div>
				<div class="modal-footer">
					<button type="button" class="btn btn-light" onclick={() => showModal = false}>Cancel</button>
					<button type="submit" class="btn btn-primary">Save Changes</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	.page-container {
		animation: fadeIn 0.3s ease-out;
	}

	@keyframes fadeIn {
		from { opacity: 0; transform: translateY(10px); }
		to { opacity: 1; transform: translateY(0); }
	}

	.table-header-actions {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
		gap: 1rem;
	}

	.search-box {
		background: white;
		border-radius: 0.5rem;
		display: flex;
		align-items: center;
		padding: 0 1rem;
		border: 1px solid #e2e8f0;
		flex: 1;
		max-width: 400px;
	}

	.search-box input {
		border: none;
		padding: 0.75rem;
		width: 100%;
		outline: none;
		font-family: inherit;
		font-weight: 600;
	}

	.search-box .material-icons-outlined { color: #7c8db5; }

	.btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		border-radius: 0.5rem;
		font-weight: 700;
		cursor: pointer;
		border: none;
		transition: all 0.2s;
	}

	.btn-primary { background: #435ebe; color: white; }
	.btn-primary:hover { background: #364b9a; }
	.btn-light { background: #f1f5f9; color: #25396f; }

	.card {
		background: white;
		border-radius: 0.75rem;
		box-shadow: 0 0 20px rgba(0,0,0,0.05);
		overflow: hidden;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
		text-align: left;
	}

	.data-table th {
		padding: 1rem 1.5rem;
		background: #f8fafc;
		color: #7c8db5;
		font-size: 0.85rem;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.data-table td {
		padding: 1rem 1.5rem;
		border-bottom: 1px solid #f1f5f9;
		color: #25396f;
		font-weight: 600;
	}

	.user-info { display: flex; align-items: center; gap: 0.75rem; }
	.avatar-small {
		width: 32px;
		height: 32px;
		background: #ebf3ff;
		color: #435ebe;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 800;
	}

	.status-pill {
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 700;
		text-transform: capitalize;
	}

	.status-pill.active { background: #dcfce7; color: #16a34a; }
	.status-pill.inactive { background: #fee2e2; color: #dc2626; }
	.status-pill.pending { background: #fef3c7; color: #d97706; }

	.badge-outline {
		padding: 0.2rem 0.6rem;
		border: 1px solid #e2e8f0;
		border-radius: 6px;
		font-size: 0.8rem;
		color: #7c8db5;
	}

	.icon-btn {
		background: none;
		border: none;
		cursor: pointer;
		padding: 0.4rem;
		border-radius: 6px;
		transition: background 0.2s;
	}

	.icon-btn.edit { color: #435ebe; }
	.icon-btn.delete { color: #ef4444; }
	.icon-btn:hover { background: #f1f5f9; }

	.text-right { text-align: right; }
	.loading-state { padding: 4rem; text-align: center; color: #7c8db5; font-weight: 600; }

	/* Modal */
	.modal-backdrop {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: rgba(0,0,0,0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 2000;
		backdrop-filter: blur(4px);
	}

	.modal {
		background: white;
		border-radius: 1rem;
		width: 100%;
		max-width: 500px;
		box-shadow: 0 20px 25px -5px rgba(0,0,0,0.1);
	}

	.modal-header {
		padding: 1.5rem;
		border-bottom: 1px solid #f1f5f9;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.modal-header h3 { margin: 0; color: #25396f; }
	.close-btn { background: none; border: none; font-size: 1.5rem; cursor: pointer; color: #7c8db5; }

	.modal-body { padding: 1.5rem; }
	.modal-footer { padding: 1.25rem 1.5rem; border-top: 1px solid #f1f5f9; display: flex; justify-content: flex-end; gap: 1rem; }

	.form-group { margin-bottom: 1.25rem; }
	label { display: block; font-weight: 700; font-size: 0.9rem; color: #25396f; margin-bottom: 0.5rem; }
	input, select {
		width: 100%;
		padding: 0.75rem;
		border: 2px solid #f1f5f9;
		border-radius: 0.5rem;
		font-family: inherit;
		font-weight: 600;
	}
	input:focus, select:focus { outline: none; border-color: #435ebe; }
</style>
