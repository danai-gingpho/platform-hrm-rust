<script lang="ts">
	let email = $state('admin@platform.com');
	let password = $state('password123');
	let isLoading = $state(false);
	let errorMessage = $state('');

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
                const data = await response.json();
                throw new Error(data.message || 'Authentication failed');
            }

            const data = await response.json();
			// Store token in cookie/localStorage
            document.cookie = `owner_token=${data.access_token}; path=/; max-age=86400`;
			
			window.location.href = '/dashboard';
		} catch (err: any) {
			errorMessage = err.message;
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="owner-auth-container">
	<div class="owner-auth-card">
        <div class="logo">
            <span class="highlight">Owner</span>Console
        </div>
		<h1>Staff Login</h1>
		<p class="subtitle">Platform Management System</p>

		<form onsubmit={handleLogin}>
			<div class="form-group">
				<label for="email">Staff Email</label>
				<input
					type="email"
					id="email"
					bind:value={email}
					required
				/>
			</div>

			<div class="form-group">
				<label for="password">Password</label>
				<input
					type="password"
					id="password"
					bind:value={password}
					required
				/>
			</div>

			{#if errorMessage}
				<p class="error">{errorMessage}</p>
			{/if}

			<button type="submit" disabled={isLoading}>
				{isLoading ? 'Authenticating...' : 'Sign In to Console'}
			</button>
		</form>
	</div>
</div>

<style>
	.owner-auth-container {
		display: flex;
		justify-content: center;
		align-items: center;
		min-height: 100vh;
		background: #0f172a; /* Dark professional theme */
	}

	.owner-auth-card {
		background: white;
		padding: 3rem;
		border-radius: 0.75rem;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		width: 100%;
		max-width: 420px;
	}

    .logo {
        font-size: 1.5rem;
        font-weight: 800;
        color: #1e293b;
        margin-bottom: 2rem;
        text-align: center;
    }

    .logo .highlight {
        color: #3b82f6;
    }

	h1 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #0f172a;
		margin-bottom: 0.25rem;
        text-align: center;
	}

	.subtitle {
		color: #64748b;
		margin-bottom: 2rem;
        text-align: center;
        font-size: 0.875rem;
	}

	.form-group {
		margin-bottom: 1.25rem;
	}

	label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		color: #475569;
		margin-bottom: 0.5rem;
        text-transform: uppercase;
        letter-spacing: 0.025em;
	}

	input {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #cbd5e1;
		border-radius: 0.375rem;
		font-size: 1rem;
		transition: all 0.2s;
	}

	input:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	button {
		width: 100%;
		padding: 0.875rem;
		background: #0f172a;
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.2s;
		margin-top: 1rem;
	}

	button:hover:not(:disabled) {
		background: #1e293b;
	}

	button:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.error {
		color: #ef4444;
		font-size: 0.875rem;
		margin-bottom: 1rem;
        text-align: center;
        background: #fef2f2;
        padding: 0.5rem;
        border-radius: 0.25rem;
	}
</style>
