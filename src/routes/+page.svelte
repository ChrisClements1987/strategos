<script lang="ts">
  import PortfolioList from '$lib/components/PortfolioList.svelte';
</script>

<main>
  <div class="container">
    <header>
      <h1>Strategos</h1>
      <p>Portfolio Management</p>
    </header>

    <section>
      <h2>Your Portfolios</h2>
      <PortfolioList />
    </section>
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    background-color: #f5f5f5;
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }

  header {
    margin-bottom: 2rem;
  }

  h1 {
    margin: 0;
    font-size: 2.5rem;
    color: #1976d2;
  }

  header p {
    margin: 0.5rem 0 0;
    color: #666;
  }

  section h2 {
    margin-bottom: 1rem;
    color: #333;
  }
</style>
