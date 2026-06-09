import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

/**
 * Type definition for User
 */
export interface User {
    id: string;
    email: string;
    roles: string[];
}

/**
 * Auth state interface
 */
interface AuthState {
    user: User | null;
    token: string | null;
    loading: boolean;
}

// Initial state
const initialState: AuthState = {
    user: null,
    token: browser ? localStorage.getItem('auth_token') : null,
    loading: true
};

// Create the store
const authStore = writable<AuthState>(initialState);

/**
 * Derived store to check if user is authenticated
 */
export const isAuthenticated = derived(authStore, ($auth) => !!$auth.token);

/**
 * Derived store for user object
 */
export const user = derived(authStore, ($auth) => $auth.user);

/**
 * Auth actions
 */
export const auth = {
    subscribe: authStore.subscribe,
    
    setToken: (token: string) => {
        if (browser) localStorage.setItem('auth_token', token);
        authStore.update(s => ({ ...s, token }));
    },
    
    setUser: (user: User) => {
        authStore.update(s => ({ ...s, user, loading: false }));
    },
    
    logout: () => {
        if (browser) localStorage.removeItem('auth_token');
        authStore.set({ user: null, token: null, loading: false });
    },
    
    setLoading: (loading: boolean) => {
        authStore.update(s => ({ ...s, loading }));
    }
};
