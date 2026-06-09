<script lang="ts">
	let email = $state('');
	let password = $state('');
	let isLoading = $state(false);
	let errorMessage = $state('');

	async function handleLogin(e: SubmitEvent) {
		e.preventDefault();
		isLoading = true;
		errorMessage = '';

		try {
			// Mock API call - Integrate with your Rust Auth service here
			const response = await fetch('/api/auth/login', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ email, password })
			});

			if (!response.ok) throw new Error('Invalid credentials');

			// Handle successful login (e.g., redirect to dashboard)
			window.location.href = '/dashboard';
		} catch (err: any) {
			errorMessage = err.message;
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="auth-container">
	<div class="auth-card">
		<h1>Welcome Back</h1>
		<p class="subtitle">Please enter your details to sign in</p>

		<form onsubmit={handleLogin}>
			<div class="form-group">
				<label for="email">Email</label>
				<input
					type="email"
					id="email"
					bind:value={email}
					placeholder="name@company.com"
					required
				/>
			</div>

			<div class="form-group">
				<label for="password">Password</label>
				<input
					type="password"
					id="password"
					bind:value={password}
					placeholder="••••••••"
					required
				/>
			</div>

			{#if errorMessage}
				<p class="error">{errorMessage}</p>
			{/if}

			<button type="submit" disabled={isLoading}>
				{isLoading ? 'Signing in...' : 'Sign In'}
			</button>
		</form>

		<p class="footer">
			Access restricted to authorized personnel only.
		</p>
	</div>
</div>

<style>
	.auth-container {
		display: flex;
		justify-content: center;
		align-items: center;
		min-height: 100vh;
		background: #f8fafc;
	}

	.auth-card {
		background: white;
		padding: 2.5rem;
		border-radius: 1rem;
		box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.1);
		width: 100%;
		max-width: 400px;
	}

	h1 {
		font-size: 1.875rem;
		font-weight: 700;
		color: #1e293b;
		margin-bottom: 0.5rem;
	}

	.subtitle {
		color: #64748b;
		margin-bottom: 2rem;
	}

	.form-group {
		margin-bottom: 1.25rem;
	}

	label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #334155;
		margin-bottom: 0.5rem;
	}

	input {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #e2e8f0;
		border-radius: 0.5rem;
		font-size: 1rem;
		transition: border-color 0.2s;
	}

	input:focus {
		outline: none;
		border-color: #3b82f6;
		ring: 2px solid #bfdbfe;
	}

	button {
		width: 100%;
		padding: 0.75rem;
		background: #2563eb;
		color: white;
		border: none;
		border-radius: 0.5rem;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.2s;
		margin-top: 1rem;
	}

	button:hover:not(:disabled) {
		background: #1d4ed8;
	}

	button:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.error {
		color: #ef4444;
		font-size: 0.875rem;
		margin-bottom: 1rem;
	}

	.footer {
		margin-top: 1.5rem;
		text-align: center;
		font-size: 0.875rem;
		color: #64748b;
	}

	a {
		color: #2563eb;
		text-decoration: none;
		font-weight: 500;
	}

	a:hover {
		text-decoration: underline;
	}
</style>
