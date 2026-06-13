export interface Tenant {
	id: string;
	name: string;
	domain: string;
	plan: 'starter' | 'business' | 'enterprise';
	status: 'active' | 'suspended' | 'trial';
	createdAt: string;
	db_schema: string;
}

export interface PlatformUser {
	id: string;
	email: string;
	tenant_id: string;
	tenant_name: string;
	global_role: 'platform_admin' | 'tenant_owner' | 'tenant_user';
	status: 'active' | 'inactive';
}
