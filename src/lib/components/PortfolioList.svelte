<script lang="ts">
  import { onMount } from 'svelte';
  import { portfolioApi } from '$lib/api/portfolio';
  import type { Portfolio } from '$lib/types/portfolio';

  let portfolios: Portfolio[] = [];
  let loading = true;
  let error: string | null = null;

  async function loadPortfolios() {
    try {
      loading = true;
      error = null;
      portfolios = await portfolioApi.getAll();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to load portfolios';
      console.error('Error loading portfolios:', err);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    loadPortfolios();
  });
</script>

{#if loading}
  <div class="loading">Loading portfolios...</div>
{:else if error}
  <div class="error">Error: {error}</div>
{:else if portfolios.length === 0}
  <div class="empty">No portfolios found. Create one to get started!</div>
{:else}
  <div class="portfolio-list">
    {#each portfolios as portfolio (portfolio.id)}
      <div class="portfolio-card">
        <h3>{portfolio.name}</h3>
        {#if portfolio.description}
          <p>{portfolio.description}</p>
        {/if}
      </div>
    {/each}
  </div>
{/if}

<style>
  .loading,
  .error,
  .empty {
    padding: 2rem;
    text-align: center;
  }

  .error {
    color: #d32f2f;
  }

  .portfolio-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1rem;
    padding: 1rem;
  }

  .portfolio-card {
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    padding: 1.5rem;
    background: white;
    transition: box-shadow 0.2s;
  }

  .portfolio-card:hover {
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }

  .portfolio-card h3 {
    margin: 0 0 0.5rem 0;
    color: #1976d2;
  }

  .portfolio-card p {
    margin: 0;
    color: #666;
  }
</style>
