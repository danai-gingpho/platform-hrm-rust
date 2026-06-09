<script lang="ts">
	let systemStatus = $state({ status: 'Loading...', version: '' });

	async fn fetchStatus() {
		try {
			const response = await fetch('/api/v1/owner/status');
			if (response.ok) {
				systemStatus = await response.json();
			}
		} catch (e) {
			systemStatus.status = 'Offline';
		}
	}

	$effect(() => {
		fetchStatus();
	});
</script>

<div class="status-indicator">
	<span class="dot" class:online={systemStatus.status === 'healthy'}></span>
	<span class="text">System: {systemStatus.status} {systemStatus.version}</span>
</div>

<style>
	.status-indicator {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.75rem;
		color: #64748b;
		padding: 0.5rem 1rem;
		background: #f1f5f9;
		border-radius: 9999px;
	}

	.dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #cbd5e1;
	}

	.dot.online {
		background: #10b981;
		box-shadow: 0 0 8px #10b981;
	}

	.text {
		font-weight: 500;
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}
</style>
