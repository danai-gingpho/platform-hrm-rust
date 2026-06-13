<script lang="ts">
	import { auth } from '$lib/stores/auth';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';

	let email = $state('');
	let password = $state('');
	let isLoading = $state(false);
	let errorMessage = $state('');

	const isSessionExpired = $page.url.searchParams.get('error') === 'session_expired';

	async function handleLogin(e: SubmitEvent) {
		e.preventDefault();
		isLoading = true;
		errorMessage = '';

		try {
			const response = await fetch('/api/v1/owner/login', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ email, password })
			});

			if (!response.ok) {
				const errorData = await response.json();
				throw new Error(errorData.message || 'Invalid credentials');
			}

			const data = await response.json();
			
			// Save token
			auth.setToken(data.access_token);
			document.cookie = `auth_token=${data.access_token}; path=/; max-age=86400; SameSite=Strict`;
			
			// Redirect
			goto('/dashboard');
		} catch (err: any) {
			errorMessage = err.message;
		} finally {
			isLoading = false;
		}
	}
</script>

<svelte:head>
	<link href="https://fonts.googleapis.com/css2?family=Nunito:wght@300;400;600;700;800&display=swap" rel="stylesheet">
</svelte:head>

<div class="login-container">
	<div class="login-split">
		<!-- Left Side: Visual/Branding -->
		<div class="login-visual">
			<div class="visual-content">
				<div class="logo-large">Voler</div>
				<h1>Simplify your <br>Workforce Management.</h1>
				<p>The most powerful and flexible HRM platform for modern enterprises.</p>
			</div>
			<div class="visual-footer">
				© 2026 Voler HRM Platform
			</div>
		</div>

		<!-- Right Side: Login Form -->
		<div class="login-form-area">
			<div class="form-wrapper">
				<div class="form-header">
					<h2>Welcome Back</h2>
					<p>Please enter your credentials to access your account.</p>
				</div>

				{#if isSessionExpired}
					<div class="alert warning">
						<span class="material-icons-outlined">info</span>
						Your session has expired. Please log in again.
					</div>
				{/if}

				{#if errorMessage}
					<div class="alert error">
						<span class="material-icons-outlined">error_outline</span>
						{errorMessage}
					</div>
				{/if}

				<form onsubmit={handleLogin}>
					<div class="form-group">
						<label for="email">Email Address</label>
						<div class="input-wrapper">
							<input
								type="email"
								id="email"
								bind:value={email}
								placeholder="name@company.com"
								required
								autocomplete="email"
							/>
						</div>
					</div>

					<div class="form-group">
						<div class="label-row">
							<label for="password">Password</label>
							<a href="/forgot-password" class="forgot-link">Forgot Password?</a>
						</div>
						<div class="input-wrapper">
							<input
								type="password"
								id="password"
								bind:value={password}
								placeholder="••••••••"
								required
								autocomplete="current-password"
							/>
						</div>
					</div>

					<div class="form-options">
						<label class="checkbox-container">
							<input type="checkbox" checked>
							<span class="checkmark"></span>
							Keep me logged in
						</label>
					</div>

					<button type="submit" class="submit-btn" disabled={isLoading}>
						{#if isLoading}
							<span class="spinner"></span>
							Signing in...
						{:else}
							Sign In
						{/if}
					</button>
				</form>

				<div class="form-footer">
					<p>Don't have an account? <a href="/register">Request access</a></p>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		font-family: 'Nunito', sans-serif;
		background: #f2f7ff;
	}

	.login-container {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.login-split {
		display: flex;
		width: 100%;
		max-width: 1100px;
		height: 700px;
		background: white;
		border-radius: 1.5rem;
		box-shadow: 0 25px 50px -12px rgba(67, 94, 190, 0.15);
		overflow: hidden;
		margin: 2rem;
	}

	/* Visual Side */
	.login-visual {
		flex: 1.2;
		background: linear-gradient(135deg, #435ebe 0%, #25396f 100%);
		padding: 4rem;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		color: white;
		position: relative;
		overflow: hidden;
	}

	.login-visual::after {
		content: '';
		position: absolute;
		bottom: -100px;
		right: -100px;
		width: 400px;
		height: 400px;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 50%;
	}

	.logo-large {
		font-size: 2.5rem;
		font-weight: 800;
		margin-bottom: 4rem;
		letter-spacing: -1px;
	}

	.visual-content h1 {
		font-size: 3rem;
		font-weight: 800;
		line-height: 1.1;
		margin-bottom: 1.5rem;
	}

	.visual-content p {
		font-size: 1.1rem;
		opacity: 0.8;
		max-width: 400px;
		line-height: 1.6;
	}

	.visual-footer {
		font-size: 0.9rem;
		opacity: 0.6;
	}

	/* Form Side */
	.login-form-area {
		flex: 1;
		padding: 4rem;
		display: flex;
		align-items: center;
		background: white;
	}

	.form-wrapper {
		width: 100%;
		max-width: 400px;
		margin: 0 auto;
	}

	.form-header h2 {
		font-size: 2rem;
		font-weight: 800;
		color: #25396f;
		margin-bottom: 0.5rem;
	}

	.form-header p {
		color: #7c8db5;
		margin-bottom: 2.5rem;
		font-weight: 600;
	}

	/* Alerts */
	.alert {
		padding: 1rem;
		border-radius: 0.75rem;
		font-size: 0.9rem;
		font-weight: 600;
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
	}

	.alert.warning { background: #fffbeb; color: #b45309; }
	.alert.error { background: #fef2f2; color: #b91c1c; }

	/* Form Groups */
	.form-group {
		margin-bottom: 1.5rem;
	}

	.label-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
	}

	label {
		display: block;
		font-weight: 700;
		font-size: 0.9rem;
		color: #25396f;
		margin-bottom: 0.5rem;
	}

	input[type="email"],
	input[type="password"] {
		width: 100%;
		padding: 0.875rem 1.25rem;
		border: 2px solid #f1f5f9;
		border-radius: 0.75rem;
		font-family: inherit;
		font-size: 1rem;
		font-weight: 600;
		transition: all 0.2s;
		background: #f8fafc;
	}

	input:focus {
		outline: none;
		border-color: #435ebe;
		background: white;
		box-shadow: 0 0 0 4px rgba(67, 94, 190, 0.1);
	}

	.forgot-link {
		font-size: 0.85rem;
		color: #435ebe;
		text-decoration: none;
		font-weight: 700;
	}

	/* Checkbox Custom */
	.form-options {
		margin-bottom: 2rem;
	}

	.checkbox-container {
		display: flex;
		align-items: center;
		position: relative;
		padding-left: 30px;
		cursor: pointer;
		font-size: 0.9rem;
		font-weight: 600;
		color: #7c8db5;
		user-select: none;
	}

	.checkbox-container input {
		position: absolute;
		opacity: 0;
		cursor: pointer;
		height: 0;
		width: 0;
	}

	.checkmark {
		position: absolute;
		top: 0;
		left: 0;
		height: 20px;
		width: 20px;
		background-color: #f1f5f9;
		border-radius: 6px;
		transition: all 0.2s;
	}

	.checkbox-container:hover input ~ .checkmark {
		background-color: #e2e8f0;
	}

	.checkbox-container input:checked ~ .checkmark {
		background-color: #435ebe;
	}

	.checkmark:after {
		content: "";
		position: absolute;
		display: none;
		left: 7px;
		top: 3px;
		width: 5px;
		height: 10px;
		border: solid white;
		border-width: 0 2px 2px 0;
		transform: rotate(45deg);
	}

	.checkbox-container input:checked ~ .checkmark:after {
		display: block;
	}

	/* Submit Button */
	.submit-btn {
		width: 100%;
		padding: 1rem;
		background: #435ebe;
		color: white;
		border: none;
		border-radius: 0.75rem;
		font-size: 1rem;
		font-weight: 700;
		cursor: pointer;
		transition: all 0.3s;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		box-shadow: 0 10px 15px -3px rgba(67, 94, 190, 0.3);
	}

	.submit-btn:hover:not(:disabled) {
		background: #364b9a;
		transform: translateY(-2px);
		box-shadow: 0 15px 20px -5px rgba(67, 94, 190, 0.4);
	}

	.submit-btn:active:not(:disabled) {
		transform: translateY(0);
	}

	.submit-btn:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.spinner {
		width: 18px;
		height: 18px;
		border: 3px solid rgba(255,255,255,0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.form-footer {
		margin-top: 2.5rem;
		text-align: center;
		font-size: 0.95rem;
		color: #7c8db5;
		font-weight: 600;
	}

	.form-footer a {
		color: #435ebe;
		text-decoration: none;
		font-weight: 800;
	}

	/* Responsive */
	@media (max-width: 992px) {
		.login-visual { display: none; }
		.login-split { max-width: 500px; height: auto; }
		.login-form-area { padding: 3rem; }
	}

	@media (max-width: 576px) {
		.login-split { margin: 1rem; }
		.login-form-area { padding: 2rem; }
	}
</style>
