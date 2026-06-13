/**
 * Staff / Employee Types
 */
export interface Staff {
	id: string;
	name: string;
	email: string;
	role: string;
	status: 'active' | 'inactive' | 'pending';
	joinedDate: string;
	avatar?: string;
}

/**
 * RBAC Types
 */
export interface Permission {
	id: string;
	name: string;
	code: string;
	description: string;
}

export interface Role {
	id: string;
	name: string;
	description: string;
	permissions: string[]; // Array of Permission IDs
}
