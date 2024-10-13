<!-- Dashboard.svelte -->
<script>
    import { onMount } from 'svelte';
    let threatData = [];
  
    onMount(async () => {
      const res = await fetch("/api/detect_threats", { method: "POST", body: JSON.stringify({type: "malware"}) });
      const data = await res.json();
      threatData = data.threat_score;
    });
  </script>
  
  <div>
    <h2>Threat Score: {threatData}</h2>
  </div>
  