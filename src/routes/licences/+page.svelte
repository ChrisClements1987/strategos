<script lang="ts">
  import { onMount } from 'svelte';
  import { portfolioApi } from '$lib/api/portfolio';
  import { licenceApi } from '$lib/api/licence';
  import type { Portfolio } from '$lib/types/portfolio';
  import type { Licence } from '$lib/types/licence';

  let portfolios: Portfolio[] = $state([]);
  let selectedPortfolio: number | null = $state(null);
  let licences: Licence[] = $state([]);
  let loading = $state(true);
  let error: string | null = $state(null);

  async function loadPortfolios() {
    try {
      portfolios = await portfolioApi.getAll();
      if (portfolios.length > 0 && !selectedPortfolio) {
        selectedPortfolio = portfolios[0].id!;
        await loadLicences(selectedPortfolio);
      }
    } catch (err) {
      error = 'Failed to load portfolios';
      console.error(err);
    } finally {
      loading = false;
    }
  }

  async function loadLicences(portfolioId: number) {
    try {
      loading = true;
      error = null;
      licences = await licenceApi.getByPortfolio(portfolioId);
    } catch (err) {
      error = 'Failed to load licences';
      console.error(err);
    } finally {
      loading = false;
    }
  }

  function handlePortfolioChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    selectedPortfolio = parseInt(target.value);
    if (selectedPortfolio) {
      loadLicences(selectedPortfolio);
    }
  }

  onMount(() => {
    loadPortfolios();
  });
</script>

<div class="page-header">
  <h1>Licences</h1>
  <p>Manage product licences and feature bundles</p>
</div>

{#if portfolios.length > 0}
  <div class="filters">
    <label>
      <span>Portfolio:</span>
      <select value={selectedPortfolio} onchange={handlePortfolioChange}>
        {#each portfolios as portfolio}
          <option value={portfolio.id}>{portfolio.name}</option>
        {/each}
      </select>
    </label>
  </div>
{/if}

{#if loading}
  <div class="loading">Loading licences...</div>
{:else if error}
  <div class="error">{error}</div>
{:else if licences.length === 0}
  <div class="empty">No licences found for this portfolio</div>
{:else}
  <div class="licence-grid">
    {#each licences as licence}
      <div class="licence-card">
        <div class="licence-header">
          <span class="licence-type">{licence.licence_type}</span>
          <span class="status" class:active={licence.status === 'active'}>{licence.status}</span>
        </div>
        <h3>{licence.name}</h3>
        {#if licence.description}
          <p>{licence.description}</p>
        {/if}
      </div>
    {/each}
  </div>
{/if}

<style>
  .page-header {
    margin-bottom: 2rem;
  }

  h1 {
    margin: 0;
    font-size: 2rem;
    color: #1e293b;
  }

  p {
    margin: 0.5rem 0 0;
    color: #64748b;
  }

  .filters {
    margin-bottom: 2rem;
    padding: 1rem;
    background: white;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .filters label {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .filters span {
    font-weight: 500;
    color: #475569;
  }

  select {
    padding: 0.5rem 1rem;
    border: 1px solid #cbd5e1;
    border-radius: 4px;
    font-size: 1rem;
    background: white;
    cursor: pointer;
  }

  .loading,
  .error,
  .empty {
    padding: 3rem;
    text-align: center;
    color: #64748b;
  }

  .error {
    color: #dc2626;
  }

  .licence-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 1.5rem;
  }

  .licence-card {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    border-left: 4px solid #f59e0b;
    transition: box-shadow 0.2s;
  }

  .licence-card:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .licence-header {
    display: flex;
    justify-content: space-between;
    margin-bottom: 0.75rem;
  }

  .licence-type {
    display: inline-block;
    padding: 0.25rem 0.75rem;
    background-color: #fef3c7;
    color: #92400e;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .status {
    padding: 0.25rem 0.75rem;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
    background-color: #fee2e2;
    color: #991b1b;
  }

  .status.active {
    background-color: #dcfce7;
    color: #166534;
  }

  h3 {
    margin: 0 0 0.5rem 0;
    color: #1e293b;
    font-size: 1.25rem;
  }

  .licence-card p {
    margin: 0;
    color: #64748b;
    font-size: 0.875rem;
  }
</style>
