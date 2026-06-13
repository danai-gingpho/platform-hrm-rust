<script lang="ts">
	const stats = [
		{ label: 'BALANCE', value: '$50', color: '#435ebe' },
		{ label: 'REVENUE', value: '$532,2', color: '#435ebe' },
		{ label: 'ORDERS', value: '1,544', color: '#435ebe' },
		{ label: 'SALES TODAY', value: '423', color: '#435ebe' }
	];
</script>

<div class="dashboard-grid">
	<!-- Stat Cards -->
	{#each stats as stat}
		<div class="stat-card">
			<div class="stat-content">
				<span class="stat-label">{stat.label}</span>
				<span class="stat-value">{stat.value}</span>
			</div>
			<div class="stat-chart">
				<!-- Placeholder for the wave chart -->
				<svg viewBox="0 0 100 40" class="wave">
					<path 
						d="M0 30 Q 12.5 10 25 30 T 50 30 T 75 30 T 100 30" 
						fill="none" 
						stroke="rgba(255,255,255,0.4)" 
						stroke-width="2"
					/>
				</svg>
			</div>
		</div>
	{/each}

	<!-- Main Sales Chart -->
	<div class="card main-chart-card">
		<div class="card-header">
			<h3>Sales</h3>
		</div>
		<div class="card-body">
			<div class="price-info">
				<span class="price">$21,102</span>
				<span class="trend up">
					<span class="material-icons-outlined">trending_up</span>
					15% than last month
				</span>
			</div>
			<div class="chart-legend">
				<div class="legend-item">
					<span class="dot last-month"></span>
					Last Month
				</div>
				<div class="legend-item">
					<span class="dot current-month"></span>
					Current Month
				</div>
			</div>
			<!-- Bar Chart Placeholder -->
			<div class="bar-chart">
				{#each Array(7) as _, i}
					<div class="bar-container">
						<div class="bar bar-bg" style="height: {30 + Math.random() * 50}%"></div>
						<div class="bar bar-fill" style="height: {10 + Math.random() * 70}%"></div>
						<span class="bar-label">{['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul'][i]}</span>
					</div>
				{/each}
			</div>
		</div>
	</div>

	<!-- Earnings Chart -->
	<div class="card earnings-card">
		<div class="card-header">
			<h3>Your Earnings</h3>
		</div>
		<div class="card-body centered">
			<div class="circular-chart">
				<svg viewBox="0 0 36 36" class="circular">
					<path class="circle-bg" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" />
					<path class="circle" stroke-dasharray="75, 100" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" />
				</svg>
				<div class="chart-info">
					<span class="label">Earnings</span>
					<span class="amount">$4,124</span>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.dashboard-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1.5rem;
	}

	/* Stat Cards */
	.stat-card {
		background: #435ebe;
		border-radius: 0.75rem;
		padding: 1.5rem;
		color: white;
		position: relative;
		overflow: hidden;
		min-height: 120px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		box-shadow: 0 10px 15px -3px rgba(67, 94, 190, 0.2);
	}

	.stat-label {
		font-size: 0.8rem;
		font-weight: 700;
		opacity: 0.8;
		letter-spacing: 0.5px;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		margin-top: 0.25rem;
		display: block;
	}

	.stat-chart {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		height: 50%;
	}

	.wave {
		width: 100%;
		height: 100%;
	}

	/* Generic Card */
	.card {
		background: white;
		border-radius: 0.75rem;
		padding: 1.5rem;
		box-shadow: 0 0 20px rgba(0,0,0,0.05);
	}

	.card-header h3 {
		margin: 0 0 1.5rem 0;
		font-size: 1.1rem;
		color: #25396f;
		font-weight: 700;
	}

	/* Main Chart Card */
	.main-chart-card {
		grid-column: span 3;
	}

	.price-info {
		display: flex;
		align-items: baseline;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.price-info .price {
		font-size: 1.75rem;
		font-weight: 800;
		color: #25396f;
	}

	.trend {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.85rem;
		font-weight: 700;
	}

	.trend.up { color: #10b981; }

	.chart-legend {
		display: flex;
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.85rem;
		font-weight: 600;
		color: #7c8db5;
	}

	.dot {
		width: 10px;
		height: 10px;
		border-radius: 50%;
	}

	.dot.last-month { background: #ebf3ff; }
	.dot.current-month { background: #435ebe; }

	.bar-chart {
		display: flex;
		align-items: flex-end;
		justify-content: space-between;
		height: 200px;
		padding-top: 1rem;
		border-bottom: 1px solid #f1f5f9;
	}

	.bar-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		width: 10%;
		height: 100%;
		position: relative;
	}

	.bar {
		width: 25px;
		border-radius: 4px 4px 0 0;
		position: absolute;
		bottom: 0;
	}

	.bar-bg {
		background: #ebf3ff;
		z-index: 1;
	}

	.bar-fill {
		background: #435ebe;
		z-index: 2;
	}

	.bar-label {
		position: absolute;
		bottom: -25px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #a0aec0;
	}

	/* Earnings Card */
	.earnings-card {
		grid-column: span 1;
	}

	.centered {
		display: flex;
		justify-content: center;
		align-items: center;
		height: 100%;
		padding: 2rem 0;
	}

	.circular-chart {
		position: relative;
		width: 180px;
		height: 180px;
	}

	.circular {
		width: 100%;
		height: 100%;
	}

	.circle-bg {
		fill: none;
		stroke: #ebf3ff;
		stroke-width: 3.8;
	}

	.circle {
		fill: none;
		stroke: #435ebe;
		stroke-width: 2.8;
		stroke-linecap: round;
		transition: stroke-dasharray 0.3s ease;
	}

	.chart-info {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		text-align: center;
		display: flex;
		flex-direction: column;
	}

	.chart-info .label {
		font-size: 0.85rem;
		font-weight: 600;
		color: #7c8db5;
	}

	.chart-info .amount {
		font-size: 1.5rem;
		font-weight: 800;
		color: #25396f;
	}

	@media (max-width: 1400px) {
		.dashboard-grid {
			grid-template-columns: repeat(2, 1fr);
		}
		.main-chart-card { grid-column: span 2; }
	}

	@media (max-width: 768px) {
		.dashboard-grid {
			grid-template-columns: 1fr;
		}
		.main-chart-card, .earnings-card { grid-column: span 1; }
	}
</style>
