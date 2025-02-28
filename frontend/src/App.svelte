<script>
    import { onMount } from 'svelte';

    let inputData = '';
    let result = '';
    let loading = false;

    // Function to analyze the threat
    async function analyzeThreat() {
        loading = true;
        result = '';  // Reset result

        try {
            const response = await fetch('http://localhost:8080/api/analyze', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ data: inputData }),
            });

            // Check if the response is OK (status 200)
            if (response.ok) {
                const jsonData = await response.json();
                result = JSON.stringify(jsonData, null, 2);  // Format the JSON nicely
            } else {
                result = `Error: ${response.statusText || 'Unable to process threat analysis.'}`;
            }
        } catch (error) {
            result = `Error: ${error.message}`;
        } finally {
            loading = false;
        }
    }
</script>

<main>
    <h1>AI-Driven Zero Trust Threat Analysis</h1>
    
    <div class="input-section">
        <label for="inputData">Enter Threat Data (JSON format):</label>
        <textarea
            id="inputData"
            bind:value={inputData}
            placeholder='{"type": "malware", "severity": "high"}'
            disabled={loading}  <!-- Disable input when analyzing -->
        ></textarea>
        <button on:click={analyzeThreat} disabled={loading || !inputData}>
            {loading ? 'Analyzing...' : 'Analyze Threat'}
        </button>
    </div>

    <div class="result-section">
        <h2>Analysis Result</h2>
        <pre>{result || 'No results yet. Enter data and click "Analyze Threat".'}</pre>
    </div>
</main>

<style>
    main {
        font-family: Arial, sans-serif;
        max-width: 600px;
        margin: 0 auto;
        padding: 1rem;
    }

    h1 {
        text-align: center;
        color: #333;
    }

    .input-section {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    textarea {
        width: 100%;
        height: 150px;
        padding: 0.5rem;
        font-size: 1rem;
        border: 1px solid #ccc;
        border-radius: 4px;
        resize: vertical;
    }

    button {
        padding: 0.7rem;
        font-size: 1rem;
        background-color: #007bff;
        color: #fff;
        border: none;
        cursor: pointer;
        border-radius: 4px;
    }

    button:disabled {
        background-color: #aaa;
        cursor: not-allowed;
    }

    .result-section {
        margin-top: 2rem;
        background-color: #f9f9f9;
        padding: 1rem;
        border-radius: 4px;
        border: 1px solid #ddd;
    }

    pre {
        font-size: 0.9rem;
        color: #333;
        white-space: pre-wrap;
        word-wrap: break-word;
    }
</style>
