<script lang="ts">
  import { onMount } from 'svelte';
  import { portfolioApi } from '$lib/api/portfolio';
  import type { Portfolio } from '$lib/types/portfolio';
  import PortfolioForm from './PortfolioForm.svelte';

  let portfolios: Portfolio[] = [];
  let loading = $state(true);
  let error: string | null = $state(null);
  let showForm = $state(false);
  let editingPortfolio: Portfolio | undefined = $state(undefined);
  let deletingId: number | null = $state(null);

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

  function handleCreate() {
    editingPortfolio = undefined;
    showForm = true;
  }

  function handleEdit(portfolio: Portfolio) {
    editingPortfolio = portfolio;
    showForm = true;
  }

  async function handleDelete(id: number) {
    if (!confirm('Are you sure you want to delete this portfolio?')) {
      return;
    }

    try {
      deletingId = id;
      await portfolioApi.delete(id);
      await loadPortfolios();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to delete portfolio';
      console.error('Error deleting portfolio:', err);
    } finally {
      deletingId = null;
    }
  }

  async function handleSave() {
    showForm = false;
    editingPortfolio = undefined;
    await loadPortfolios();
  }

  function handleCancel() {
    showForm = false;
    editingPortfolio = undefined;
  }

  onMount(() => {
    loadPortfolios();
  });
</script>

{#if showForm}
  <div class="form-container">
    <h3>{editingPortfolio ? 'Edit Portfolio' : 'Create Portfolio'}</h3>
    <PortfolioForm 
      portfolio={editingPortfolio}
      onSave={handleSave}
      onCancel={handleCancel}
    />
  </div>
{:else}
  <div class="header-actions">
    <button class="btn-create" onclick={handleCreate}>
      + Create Portfolio
    </button>
  </div>

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
          <div class="card-content">
            <h3>{portfolio.name}</h3>
            {#if portfolio.description}
              <p>{portfolio.description}</p>
            {/if}
          </div>
          <div class="card-actions">
            <button class="btn-edit" onclick={() => handleEdit(portfolio)}>
              Edit
            </button>
            <button 
              class="btn-delete" 
              onclick={() => handleDelete(portfolio.id!)}
              disabled={deletingId === portfolio.id}
            >
              {deletingId === portfolio.id ? 'Deleting...' : 'Delete'}
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
{/if}

<style>
  .form-container {
    background: white;
    padding: 2rem;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .form-container h3 {
    margin: 0 0 1.5rem 0;
    color: #333;
  }

  .header-actions {
    display: flex;
    justify-content: flex-end;
    margin-bottom: 1.5rem;
  }

  .btn-create {
    padding: 0.75rem 1.5rem;
    background-color: #1976d2;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .btn-create:hover {
    background-color: #1565c0;
  }

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
    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
    gap: 1rem;
  }

  .portfolio-card {
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    padding: 1.5rem;
    background: white;
    transition: box-shadow 0.2s;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .portfolio-card:hover {
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }

  .card-content {
    flex: 1;
  }

  .card-content h3 {
    margin: 0 0 0.5rem 0;
    color: #1976d2;
  }

  .card-content p {
    margin: 0;
    color: #666;
  }

  .card-actions {
    display: flex;
    gap: 0.5rem;
    justify-content: flex-end;
  }

  .btn-edit,
  .btn-delete {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 4px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-edit {
    background-color: #e3f2fd;
    color: #1976d2;
  }

  .btn-edit:hover {
    background-color: #bbdefb;
  }

  .btn-delete {
    background-color: #ffebee;
    color: #c62828;
  }

  .btn-delete:hover:not(:disabled) {
    background-color: #ffcdd2;
  }

  .btn-delete:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>
