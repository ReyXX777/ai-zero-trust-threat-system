<script>
  import { onMount } from 'svelte';
  let threatData = null;  // Initialize to null to handle loading state
  let loading = true;     // Loading state to show while waiting for the response
  let error = null;       // Error state in case of failure

  onMount(async () => {
    try {
      const res = await fetch("/api/detect_threats", {
        method: "POST",
        headers: {
          "Content-Type": "application/json", // Set the correct header
        },
        body: JSON.stringify({ type: "malware" }), // Ensure the data is sent as JSON
      });

      if (res.ok) {
        const data = await res.json();
        threatData = data.threat_score;  // Assuming response has 'threat_score'
      } else {
        throw new Error("Failed to fetch threat data");
      }
    } catch (err) {
      error = err.message;  // Handle any errors
    } finally {
      loading = false;  // Set loading to false when the request is complete
    }
  });
</script>

<div>
  {#if loading}
    <p>Loading...</p>  <!-- Show loading message -->
  {:else if error}
    <p style="color: red;">Error: {error}</p> <!-- Display error message -->
  {:else}
    <h2>Threat Score: {threatData}</h2>  <!-- Display the fetched threat score -->
  {/if}
</div>
