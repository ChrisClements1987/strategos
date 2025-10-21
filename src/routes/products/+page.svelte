<script lang="ts">
  import { onMount } from 'svelte';
  import { portfolioApi } from '$lib/api/portfolio';
  import { productApi } from '$lib/api/product';
  import type { Portfolio } from '$lib/types/portfolio';
  import type { Product } from '$lib/types/product';

  let portfolios: Portfolio[] = $state([]);
  let selectedPortfolio: number | null = $state(null);
  let products: Product[] = $state([]);
  let loading = $state(true);
  let error: string | null = $state(null);

  async function loadPortfolios() {
    try {
      portfolios = await portfolioApi.getAll();
      if (portfolios.length > 0 && !selectedPortfolio) {
        selectedPortfolio = portfolios[0].id!;
        await loadProducts(selectedPortfolio);
      }
    } catch (err) {
      error = 'Failed to load portfolios';
      console.error(err);
    } finally {
      loading = false;
    }
  }

  async function loadProducts(portfolioId: number) {
    try {
      loading = true;
      error = null;
      products = await productApi.getByPortfolio(portfolioId);
    } catch (err) {
      error = 'Failed to load products';
      console.error(err);
    } finally {
      loading = false;
    }
  }

  function handlePortfolioChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    selectedPortfolio = parseInt(target.value);
    if (selectedPortfolio) {
      loadProducts(selectedPortfolio);
    }
  }

  onMount(() => {
    loadPortfolios();
  });
</script>

<div class="page-header">
  <h1>Products</h1>
  <p>View products and modules across your portfolios</p>
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
  <div class="loading">Loading products...</div>
{:else if error}
  <div class="error">{error}</div>
{:else if products.length === 0}
  <div class="empty">No products found for this portfolio</div>
{:else}
  <div class="product-grid">
    {#each products as product}
      <div class="product-card" class:module={product.product_type === 'module'}>
        <div class="product-header">
          <span class="product-type">{product.product_type}</span>
          <span class="status" class:active={product.status === 'active'}>{product.status}</span>
        </div>
        <h3>{product.name}</h3>
        {#if product.description}
          <p>{product.description}</p>
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

  .product-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 1.5rem;
  }

  .product-card {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    border-left: 4px solid #3b82f6;
    transition: box-shadow 0.2s;
  }

  .product-card.module {
    border-left-color: #8b5cf6;
  }

  .product-card:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .product-header {
    display: flex;
    justify-content: space-between;
    margin-bottom: 0.75rem;
  }

  .product-type {
    display: inline-block;
    padding: 0.25rem 0.75rem;
    background-color: #dbeafe;
    color: #1e40af;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .module .product-type {
    background-color: #ede9fe;
    color: #6d28d9;
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

  .product-card p {
    margin: 0;
    color: #64748b;
    font-size: 0.875rem;
  }
</style>
