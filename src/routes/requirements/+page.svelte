<script lang="ts">
  import { onMount } from 'svelte';
  import { portfolioApi } from '$lib/api/portfolio';
  import { clientApi } from '$lib/api/client';
  import { requirementApi } from '$lib/api/requirement';
  import type { Portfolio } from '$lib/types/portfolio';
  import type { Client } from '$lib/types/client';
  import type { Requirement } from '$lib/types/requirement';

  let portfolios: Portfolio[] = $state([]);
  let clients: Client[] = $state([]);
  let selectedPortfolio: number | null = $state(null);
  let selectedClient: number | null = $state(null);
  let requirements: Requirement[] = $state([]);
  let loading = $state(true);

  async function loadData() {
    try {
      portfolios = await portfolioApi.getAll();
      if (portfolios.length > 0) {
        selectedPortfolio = portfolios[0].id!;
        clients = await clientApi.getByPortfolio(selectedPortfolio);
        if (clients.length > 0) {
          selectedClient = clients[0].id!;
          requirements = await requirementApi.getByClient(selectedClient);
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
    clients = await clientApi.getByPortfolio(selectedPortfolio);
    if (clients.length > 0) {
      selectedClient = clients[0].id!;
      requirements = await requirementApi.getByClient(selectedClient);
    }
  }

  async function handleClientChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    selectedClient = parseInt(target.value);
    requirements = await requirementApi.getByClient(selectedClient);
  }

  onMount(loadData);
</script>

<div class="page-header">
  <h1>Requirements</h1>
  <p>Track client requirements and feature requests</p>
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
  
  {#if clients.length > 0}
    <label>
      <span>Client:</span>
      <select value={selectedClient} onchange={handleClientChange}>
        {#each clients as client}
          <option value={client.id}>{client.name}</option>
        {/each}
      </select>
    </label>
  {/if}
</div>

{#if loading}
  <div class="loading">Loading requirements...</div>
{:else if requirements.length === 0}
  <div class="empty">No requirements found</div>
{:else}
  <div class="requirement-list">
    {#each requirements as req}
      <div class="requirement-card">
        <div class="req-header">
          <span class="priority priority-{req.priority}">{req.priority}</span>
          <span class="status status-{req.status}">{req.status}</span>
        </div>
        <h3>{req.name}</h3>
        {#if req.description}
          <p>{req.description}</p>
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

  .requirement-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .requirement-card {
    background: white;
    padding: 1.25rem;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    border-left: 4px solid #ec4899;
  }

  .req-header {
    display: flex;
    gap: 0.5rem;
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

  .status-pending { background-color: #f3f4f6; color: #6b7280; }
  .status-approved { background-color: #dbeafe; color: #1e40af; }
  .status-in_progress { background-color: #fef3c7; color: #92400e; }
  .status-completed { background-color: #dcfce7; color: #166534; }
  .status-rejected { background-color: #fee2e2; color: #991b1b; }

  h3 {
    margin: 0 0 0.5rem 0;
    color: #1e293b;
    font-size: 1.125rem;
  }

  .requirement-card p {
    margin: 0;
    color: #64748b;
    font-size: 0.875rem;
  }
</style>
