
// this file is generated — do not edit it


declare module "svelte/elements" {
	export interface HTMLAttributes<T> {
		'data-sveltekit-keepfocus'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-noscroll'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-preload-code'?:
			| true
			| ''
			| 'eager'
			| 'viewport'
			| 'hover'
			| 'tap'
			| 'off'
			| undefined
			| null;
		'data-sveltekit-preload-data'?: true | '' | 'hover' | 'tap' | 'off' | undefined | null;
		'data-sveltekit-reload'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-replacestate'?: true | '' | 'off' | undefined | null;
	}
}

export {};


declare module "$app/types" {
	type MatcherParam<M> = M extends (param : string) => param is (infer U extends string) ? U : string;

	export interface AppTypes {
		RouteId(): "/(owner)" | "/(dashboard)" | "/" | "/(dashboard)/dashboard" | "/login" | "/(owner)/owner" | "/(owner)/owner/permissions" | "/(owner)/owner/roles" | "/(owner)/owner/staff" | "/(owner)/platform" | "/(owner)/platform/tenants" | "/(owner)/platform/users";
		RouteParams(): {
			
		};
		LayoutParams(): {
			"/(owner)": Record<string, never>;
			"/(dashboard)": Record<string, never>;
			"/": Record<string, never>;
			"/(dashboard)/dashboard": Record<string, never>;
			"/login": Record<string, never>;
			"/(owner)/owner": Record<string, never>;
			"/(owner)/owner/permissions": Record<string, never>;
			"/(owner)/owner/roles": Record<string, never>;
			"/(owner)/owner/staff": Record<string, never>;
			"/(owner)/platform": Record<string, never>;
			"/(owner)/platform/tenants": Record<string, never>;
			"/(owner)/platform/users": Record<string, never>
		};
		Pathname(): "/" | "/dashboard" | "/login" | "/owner/permissions" | "/owner/roles" | "/owner/staff" | "/platform/tenants" | "/platform/users";
		ResolvedPathname(): `${"" | `/${string}`}${ReturnType<AppTypes['Pathname']>}`;
		Asset(): "/favicon.png" | string & {};
	}
}