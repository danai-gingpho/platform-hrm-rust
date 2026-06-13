import { browser } from "$app/environment";
import { goto } from "$app/navigation";

/**
 * Custom Error class to handle API-specific errors
 */
export class ApiError extends Error {
  constructor(
    public status: number,
    public message: string,
    public data?: any,
  ) {
    super(message);
    this.name = "ApiError";
  }
}

/**
 * Base configuration for the API Client
 */
const BASE_URL =
  import.meta.env.VITE_API_BASE_URL || "http://localhost:8000/api/v1";

/**
 * Core fetch wrapper with middleware-like capabilities
 */
async function request<T>(
  method: string,
  path: string,
  body?: any,
  customHeaders: Record<string, string> = {},
): Promise<T> {
  const url = `${BASE_URL}${path}`;

  // 1. Prepare Headers
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    ...customHeaders,
  };

  // 2. Inject Auth Token (Example: from localStorage if in browser)
  if (browser) {
    const token = localStorage.getItem("auth_token");
    if (token) {
      headers["Authorization"] = `Bearer ${token}`;
    }
  }

  // 3. Execute Fetch
  const response = await fetch(url, {
    method,
    headers,
    body: body ? JSON.stringify(body) : undefined,
  });

  // 4. Handle Response
  if (!response.ok) {
    // Auto-logout on 401 Unauthorized
    if (response.status === 401 && browser) {
      localStorage.removeItem("auth_token");
      document.cookie =
        "auth_token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
      goto("/login?error=session_expired");
      return {} as T;
    }

    let errorData;
    try {
      errorData = await response.json();
    } catch {
      errorData = { message: "An unexpected error occurred" };
    }

    throw new ApiError(
      response.status,
      errorData.error || errorData.message || "API Request Failed",
      errorData,
    );
  }

  // 5. Parse Success Data (Handle empty response)
  if (response.status === 204) return {} as T;
  return response.json();
}

/**
 * Exported API Methods
 */
export const api = {
  get: <T>(path: string, headers?: Record<string, string>) =>
    request<T>("GET", path, undefined, headers),

  post: <T>(path: string, body: any, headers?: Record<string, string>) =>
    request<T>("POST", path, body, headers),

  put: <T>(path: string, body: any, headers?: Record<string, string>) =>
    request<T>("PUT", path, body, headers),

  patch: <T>(path: string, body: any, headers?: Record<string, string>) =>
    request<T>("PATCH", path, body, headers),

  delete: <T>(path: string, headers?: Record<string, string>) =>
    request<T>("DELETE", path, undefined, headers),
};
