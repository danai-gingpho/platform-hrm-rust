<script lang="ts">
	import type { Tenant } from '$lib/types/platform';
	import { onMount } from 'svelte';

	let tenants = $state<Tenant[]>([]);
	let isLoading = $state(true);
	let showModal = $state(false);
	let editingTenant = $state<Tenant | null>(null);

	const mockTenants: Tenant[] = [
		{ id: '1', name: 'Acme Corp', domain: 'acme.hrm.com', plan: 'enterprise', status: 'active', createdAt: '2024-01-01', db_schema: 'tenant_acme' },
		{ id: '2', name: 'Global Tech', domain: 'global.hrm.com', plan: 'business', status: 'active', createdAt: '2024-03-15', db_schema: 'tenant_global' },
		{ id: '3', name: 'Startup Inc', domain: 'startup.hrm.com', plan: 'starter', status: 'trial', createdAt: '2024-05-20', db_schema: 'tenant_startup' }
	];

	onMount(() => {
		setTimeout(() => { tenants = mockTenants; isLoading = false; }, 600);
	});

	function handleSave(e: SubmitEvent) {
		e.preventDefault();
		const formData = new FormData(e.target as HTMLFormElement);
		const data = Object.fromEntries(formData) as any;

		if (editingTenant) {
			tenants = tenants.map(t => t.id === editingTenant!.id ? { ...t, ...data } : t);
		} else {
			const newTenant: Tenant = {
				id: Math.random().toString(36).substr(2, 9),
				createdAt: new Date().toISOString().split('T')[0],
				status: 'active',
				...data
			};
			tenants = [newTenant, ...tenants];
		}
		showModal = false;
	}
</script>

<div class="page-container">
	<div class="header-section">
		<div>
			<h1>Tenant Management</h1>
			<p>Provision and manage organization accounts (Companies)</p>
		</div>
		<button class="btn btn-primary" onclick={() => { editingTenant = null; showModal = true; }}>
			<span class="material-icons-outlined">add_business</span>
			Create New Tenant
		</button>
	</div>

	<div class="card">
		<table class="data-table">
			<thead>
				<tr>
					<th>Company Name</th>
					<th>Domain</th>
					<th>Plan</th>
					<th>Status</th>
					<th>Created At</th>
					<th class="text-right">Actions</th>
				</tr>
			</thead>
			<tbody>
				{#each tenants as tenant}
					<tr>
						<td><span class="font-bold">{tenant.name}</span><br><small>{tenant.db_schema}</small></td>
						<td><code>{tenant.domain}</code></td>
						<td><span class="plan-badge {tenant.plan}">{tenant.plan}</span></td>
						<td><span class="status-pill {tenant.status}">{tenant.status}</span></td>
						<td>{tenant.createdAt}</td>
						<td class="text-right">
							<button class="icon-btn" onclick={() => { editingTenant = tenant; showModal = true; }}>
								<span class="material-icons-outlined">edit</span>
							</button>
							<button class="icon-btn delete"><span class="material-icons-outlined">delete</span></button>
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
				<h3>{editingTenant ? 'Edit Tenant' : 'Provision New Tenant'}</h3>
				<button class="close-btn" onclick={() => showModal = false}>&times;</button>
			</div>
			<form onsubmit={handleSave}>
				<div class="modal-body">
					<div class="form-group">
						<label for="name">Company Name</label>
						<input type="text" name="name" value={editingTenant?.name || ''} required />
					</div>
					<div class="form-group">
						<label for="domain">Custom Domain / Subdomain</label>
						<input type="text" name="domain" value={editingTenant?.domain || ''} required placeholder="company.hrm.com" />
					</div>
					<div class="form-row">
						<div class="form-group">
							<label for="plan">Subscription Plan</label>
							<select name="plan" value={editingTenant?.plan || 'starter'}>
								<option value="starter">Starter</option>
								<option value="business">Business</option>
								<option value="enterprise">Enterprise</option>
							</select>
						</div>
						<div class="form-group">
							<label for="db_schema">DB Schema</label>
							<input type="text" name="db_schema" value={editingTenant?.db_schema || ''} required placeholder="tenant_name" />
						</div>
					</div>
				</div>
				<div class="modal-footer">
					<button type="button" class="btn btn-light" onclick={() => showModal = false}>Cancel</button>
					<button type="submit" class="btn btn-primary">Save Tenant</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	.header-section { display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem; }
	.header-section h1 { margin: 0; color: #25396f; font-weight: 800; }
	.header-section p { margin: 0.25rem 0 0 0; color: #7c8db5; font-weight: 600; }

	.card { background: white; border-radius: 1rem; box-shadow: 0 0 20px rgba(0,0,0,0.05); overflow: hidden; }
	.data-table { width: 100%; border-collapse: collapse; text-align: left; }
	.data-table th { padding: 1.25rem; background: #f8fafc; color: #7c8db5; font-size: 0.8rem; text-transform: uppercase; letter-spacing: 1px; }
	.data-table td { padding: 1.25rem; border-bottom: 1px solid #f1f5f9; color: #25396f; font-weight: 600; }

	.plan-badge { padding: 0.25rem 0.6rem; border-radius: 6px; font-size: 0.75rem; font-weight: 800; text-transform: uppercase; }
	.plan-badge.enterprise { background: #ebf3ff; color: #435ebe; }
	.plan-badge.business { background: #dcfce7; color: #16a34a; }
	.plan-badge.starter { background: #f1f5f9; color: #64748b; }

	.status-pill { padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.75rem; font-weight: 700; text-transform: capitalize; }
	.status-pill.active { background: #dcfce7; color: #16a34a; }
	.status-pill.trial { background: #ebf3ff; color: #435ebe; }

	.btn { display: flex; align-items: center; gap: 0.5rem; padding: 0.85rem 1.5rem; border-radius: 0.75rem; font-weight: 700; cursor: pointer; border: none; }
	.btn-primary { background: #435ebe; color: white; }
	.btn-light { background: #f1f5f9; color: #25396f; }
	.icon-btn { background: none; border: none; cursor: pointer; color: #7c8db5; padding: 0.5rem; border-radius: 8px; }
	.icon-btn:hover { background: #f1f5f9; color: #435ebe; }
	.text-right { text-align: right; }

	/* Modal Basic */
	.modal-backdrop { position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.4); display: flex; align-items: center; justify-content: center; z-index: 2000; backdrop-filter: blur(4px); }
	.modal { background: white; border-radius: 1.25rem; width: 100%; max-width: 600px; padding: 0; overflow: hidden; }
	.modal-header { padding: 1.5rem; border-bottom: 1px solid #f1f5f9; display: flex; justify-content: space-between; }
	.modal-body { padding: 1.5rem; }
	.modal-footer { padding: 1.5rem; border-top: 1px solid #f1f5f9; display: flex; justify-content: flex-end; gap: 1rem; }
	.form-group { margin-bottom: 1.25rem; }
	.form-row { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; }
	label { display: block; font-weight: 700; margin-bottom: 0.5rem; color: #25396f; }
	input, select { width: 100%; padding: 0.75rem; border: 2px solid #f1f5f9; border-radius: 0.5rem; font-family: inherit; font-weight: 600; }
	.close-btn { background: none; border: none; font-size: 1.5rem; cursor: pointer; }
</style>
