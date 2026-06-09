import type { Handle } from '@sveltejs/kit';
import { redirect } from '@sveltejs/kit';

/**
 * Global Server Hook for Authentication & Route Protection
 */
export const handle: Handle = async ({ event, resolve }) => {
    // 1. Get token from Cookies (More secure than localStorage for SSR)
    const token = event.cookies.get('auth_token');
    
    // 2. Define protected routes
    const isProtectedRoute = event.url.pathname.startsWith('/dashboard') || 
                           event.url.pathname.startsWith('/admin');

    // 3. Logic: If no token and trying to access protected route
    if (isProtectedRoute && !token) {
        // Redirect to login page
        throw redirect(303, '/login');
    }

    // 4. Optionally: Validate token with backend if it exists
    // (You can call your Auth Microservice here to verify the token)
    if (token) {
        // event.locals.user = await verifyToken(token);
    }

    // 5. Continue to the requested page
    const response = await resolve(event);
    return response;
};
