<script lang="ts">
  import { onMount } from 'svelte';
  import { portfolioApi } from '$lib/api/portfolio';
  import { productApi } from '$lib/api/product';
  import { featureApi } from '$lib/api/feature';
  import type { Portfolio } from '$lib/types/portfolio';
  import type { Product } from '$lib/types/product';
  import type { Feature } from '$lib/types/feature';

  let portfolios: Portfolio[] = $state([]);
  let products: Product[] = $state([]);
  let selectedPortfolio: number | null = $state(null);
  let selectedProduct: number | null = $state(null);
  let features: Feature[] = $state([]);
  let loading = $state(true);

  async function loadData() {
    try {
      portfolios = await portfolioApi.getAll();
      if (portfolios.length > 0) {
        selectedPortfolio = portfolios[0].id!;
        products = await productApi.getByPortfolio(selectedPortfolio);
        if (products.length > 0) {
          selectedProduct = products[0].id!;
          features = await featureApi.getByProduct(selectedProduct);
        }
      }
    } catch (err) {
      console.error(err);
    } finally {
      loading = false;
    }
  }

  async function handlePortfolioChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    selectedPortfolio = parseInt(target.value);
    products = await productApi.getByPortfolio(selectedPortfolio);
    if (products.length > 0) {
      selectedProduct = products[0].id!;
      features = await featureApi.getByProduct(selectedProduct);
    }
  }

  async function handleProductChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    selectedProduct = parseInt(target.value);
    features = await featureApi.getByProduct(selectedProduct);
  }

  onMount(loadData);
</script>

<div class="page-header">
  <h1>Features</h1>
  <p>View product features by portfolio and product</p>
</div>

<div class="filters">
  {#if portfolios.length > 0}
    <label>
      <span>Portfolio:</span>
      <select value={selectedPortfolio} onchange={handlePortfolioChange}>
        {#each portfolios as portfolio}
          <option value={portfolio.id}>{portfolio.name}</option>
        {/each}
      </select>
    </label>
  {/if}
  
  {#if products.length > 0}
    <label>
      <span>Product:</span>
      <select value={selectedProduct} onchange={handleProductChange}>
        {#each products as product}
          <option value={product.id}>{product.name}</option>
        {/each}
      </select>
    </label>
  {/if}
</div>

{#if loading}
  <div class="loading">Loading features...</div>
{:else if features.length === 0}
  <div class="empty">No features found</div>
{:else}
  <div class="feature-grid">
    {#each features as feature}
      <div class="feature-card">
        <div class="feature-header">
          <span class="priority priority-{feature.priority}">{feature.priority}</span>
          <span class="status status-{feature.status}">{feature.status}</span>
        </div>
        <h3>{feature.name}</h3>
        {#if feature.description}
          <p>{feature.description}</p>
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
    display: flex;
    gap: 1.5rem;
    margin-bottom: 2rem;
    padding: 1rem;
    background: white;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .filters label {
    display: flex;
    align-items: center;
    gap: 0.75rem;
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
  .empty {
    padding: 3rem;
    text-align: center;
    color: #64748b;
  }

  .feature-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1rem;
  }

  .feature-card {
    background: white;
    padding: 1.25rem;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    border-left: 4px solid #6366f1;
  }

  .feature-header {
    display: flex;
    justify-content: space-between;
    margin-bottom: 0.75rem;
  }

  .priority,
  .status {
    padding: 0.2rem 0.6rem;
    border-radius: 12px;
    font-size: 0.7rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .priority-low { background-color: #e0e7ff; color: #3730a3; }
  .priority-medium { background-color: #fef3c7; color: #92400e; }
  .priority-high { background-color: #fed7aa; color: #9a3412; }
  .priority-critical { background-color: #fee2e2; color: #991b1b; }

  .status-planned { background-color: #e0e7ff; color: #3730a3; }
  .status-in_progress { background-color: #fef3c7; color: #92400e; }
  .status-completed { background-color: #dcfce7; color: #166534; }
  .status-cancelled { background-color: #f3f4f6; color: #6b7280; }

  h3 {
    margin: 0 0 0.5rem 0;
    color: #1e293b;
    font-size: 1.125rem;
  }

  .feature-card p {
    margin: 0;
    color: #64748b;
    font-size: 0.875rem;
  }
</style>
