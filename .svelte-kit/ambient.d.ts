
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * _Unlike_ [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 * 
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * 
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 * 
 * You can override `.env` values from the command line like so:
 * 
 * ```bash
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const SHELL: string;
	export const npm_command: string;
	export const PYENV_SHELL: string;
	export const NVM_INC: string;
	export const HISTCONTROL: string;
	export const HISTSIZE: string;
	export const HOSTNAME: string;
	export const DOTNET_ROOT: string;
	export const npm_config_verify_deps_before_run: string;
	export const ELECTRON_RUN_AS_NODE: string;
	export const GPG_TTY: string;
	export const EDITOR: string;
	export const PWD: string;
	export const LOGNAME: string;
	export const XDG_SESSION_TYPE: string;
	export const PNPM_HOME: string;
	export const VSCODE_ESM_ENTRYPOINT: string;
	export const ELECTRON_NO_ASAR: string;
	export const MOTD_SHOWN: string;
	export const HOME: string;
	export const LANG: string;
	export const VSCODE_DOTNET_INSTALL_TOOL_ORIGINAL_HOME: string;
	export const SSL_CERT_DIR: string;
	export const NX_PROJECT_GLOB_CACHE: string;
	export const VSCODE_AGENT_FOLDER: string;
	export const VSCODE_L10N_BUNDLE_LOCATION: string;
	export const pnpm_config_verify_deps_before_run: string;
	export const SSH_CONNECTION: string;
	export const DOTNET_BUNDLE_EXTRACT_BASE_DIR: string;
	export const NVM_DIR: string;
	export const MOZ_GMP_PATH: string;
	export const XDG_SESSION_CLASS: string;
	export const VSCODE_HANDLES_SIGPIPE: string;
	export const SELINUX_ROLE_REQUESTED: string;
	export const LESSOPEN: string;
	export const USER: string;
	export const SELINUX_USE_CURRENT_RANGE: string;
	export const SHLVL: string;
	export const NVM_CD_FLAGS: string;
	export const VSCODE_CWD: string;
	export const XDG_SESSION_ID: string;
	export const npm_config_user_agent: string;
	export const PNPM_PACKAGE_NAME: string;
	export const XDG_RUNTIME_DIR: string;
	export const SSL_CERT_FILE: string;
	export const NODE_PATH: string;
	export const SSH_CLIENT: string;
	export const PYENV_ROOT: string;
	export const DEBUGINFOD_URLS: string;
	export const DEBUGINFOD_IMA_CERT_PATH: string;
	export const VSCODE_CLI_REQUIRE_TOKEN: string;
	export const XDG_DATA_DIRS: string;
	export const BROWSER: string;
	export const PATH: string;
	export const SELINUX_LEVEL_REQUESTED: string;
	export const CI: string;
	export const APPLICATION_INSIGHTS_NO_STATSBEAT: string;
	export const DBUS_SESSION_BUS_ADDRESS: string;
	export const VSCODE_NLS_CONFIG: string;
	export const NVM_BIN: string;
	export const MAIL: string;
	export const NX_WORKSPACE_ROOT_PATH: string;
	export const VSCODE_HANDLES_UNCAUGHT_ERRORS: string;
	export const VSCODE_IPC_HOOK_CLI: string;
	export const NX_CLI_SET: string;
	export const NX_CACHE_PROJECTS_CONFIG: string;
	export const NODE_ENV: string;
}

/**
 * Similar to [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Values are replaced statically at build time.
 * 
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * This module cannot be imported into client-side code.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 * 
 * > In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		SHELL: string;
		npm_command: string;
		PYENV_SHELL: string;
		NVM_INC: string;
		HISTCONTROL: string;
		HISTSIZE: string;
		HOSTNAME: string;
		DOTNET_ROOT: string;
		npm_config_verify_deps_before_run: string;
		ELECTRON_RUN_AS_NODE: string;
		GPG_TTY: string;
		EDITOR: string;
		PWD: string;
		LOGNAME: string;
		XDG_SESSION_TYPE: string;
		PNPM_HOME: string;
		VSCODE_ESM_ENTRYPOINT: string;
		ELECTRON_NO_ASAR: string;
		MOTD_SHOWN: string;
		HOME: string;
		LANG: string;
		VSCODE_DOTNET_INSTALL_TOOL_ORIGINAL_HOME: string;
		SSL_CERT_DIR: string;
		NX_PROJECT_GLOB_CACHE: string;
		VSCODE_AGENT_FOLDER: string;
		VSCODE_L10N_BUNDLE_LOCATION: string;
		pnpm_config_verify_deps_before_run: string;
		SSH_CONNECTION: string;
		DOTNET_BUNDLE_EXTRACT_BASE_DIR: string;
		NVM_DIR: string;
		MOZ_GMP_PATH: string;
		XDG_SESSION_CLASS: string;
		VSCODE_HANDLES_SIGPIPE: string;
		SELINUX_ROLE_REQUESTED: string;
		LESSOPEN: string;
		USER: string;
		SELINUX_USE_CURRENT_RANGE: string;
		SHLVL: string;
		NVM_CD_FLAGS: string;
		VSCODE_CWD: string;
		XDG_SESSION_ID: string;
		npm_config_user_agent: string;
		PNPM_PACKAGE_NAME: string;
		XDG_RUNTIME_DIR: string;
		SSL_CERT_FILE: string;
		NODE_PATH: string;
		SSH_CLIENT: string;
		PYENV_ROOT: string;
		DEBUGINFOD_URLS: string;
		DEBUGINFOD_IMA_CERT_PATH: string;
		VSCODE_CLI_REQUIRE_TOKEN: string;
		XDG_DATA_DIRS: string;
		BROWSER: string;
		PATH: string;
		SELINUX_LEVEL_REQUESTED: string;
		CI: string;
		APPLICATION_INSIGHTS_NO_STATSBEAT: string;
		DBUS_SESSION_BUS_ADDRESS: string;
		VSCODE_NLS_CONFIG: string;
		NVM_BIN: string;
		MAIL: string;
		NX_WORKSPACE_ROOT_PATH: string;
		VSCODE_HANDLES_UNCAUGHT_ERRORS: string;
		VSCODE_IPC_HOOK_CLI: string;
		NX_CLI_SET: string;
		NX_CACHE_PROJECTS_CONFIG: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * Similar to [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
