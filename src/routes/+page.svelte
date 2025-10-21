<script lang="ts">
  import PortfolioList from '$lib/components/PortfolioList.svelte';
</script>

<div class="page-header">
  <h1>Portfolios</h1>
  <p>Manage your business portfolios and strategic initiatives</p>
</div>

<PortfolioList />

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
</style>
