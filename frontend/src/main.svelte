<!-- main.svelte -->

<script>
    import { onMount } from 'svelte';

    // State variables
    let threatInput = '';
    let threatAnalysisResult = '';
    let isLoading = false;

    // Function to send threat data to the backend and fetch analysis
    async function analyzeThreat() {
        isLoading = true;
        threatAnalysisResult = '';  // Clear previous results

        try {
            const response = await fetch('http://localhost:8080/api/analyze', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ data: threatInput }),
            });

            if (response.ok) {
                const responseData = await response.json();
                threatAnalysisResult = JSON.stringify(responseData, null, 2);
            } else {
                threatAnalysisResult = 'Error: Unable to process threat analysis.';
            }
        } catch (error) {
            threatAnalysisResult = `Error: ${error.message}`;
        } finally {
            isLoading = false;
        }
    }
</script>

<main>
    <h1>AI-Driven Zero Trust Threat Analysis System</h1>

    <section class="input-section">
        <label for="threatInput">Threat Data (in JSON format):</label>
        <textarea
            id="threatInput"
            bind:value={threatInput}
            placeholder='{"type": "malware", "severity": "high"}'
            rows="8"
        ></textarea>
        <button on:click={analyzeThreat} disabled={isLoading}>
            {isLoading ? 'Analyzing...' : 'Analyze Threat'}
        </button>
    </section>

    <section class="result-section">
        <h2>Analysis Result</h2>
        <pre>{threatAnalysisResult || 'Enter threat data and click "Analyze Threat" to see the result.'}</pre>
    </section>
</main>

<style>
    main {
        font-family: Arial, sans-serif;
        max-width: 800px;
        margin: 0 auto;
        padding: 2rem;
        color: #333;
    }

    h1 {
        text-align: center;
        color: #2c3e50;
    }

    .input-section {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        margin-top: 2rem;
    }

    label {
        font-size: 1.1rem;
        color: #555;
    }

    textarea {
        width: 100%;
        padding: 0.8rem;
        font-size: 1rem;
        border-radius: 4px;
        border: 1px solid #ddd;
    }

    button {
        padding: 0.8rem;
        font-size: 1rem;
        background-color: #007bff;
        color: #fff;
        border: none;
        cursor: pointer;
        border-radius: 4px;
        transition: background-color 0.2s ease;
    }

    button:hover:not(:disabled) {
        background-color: #0056b3;
    }

    button:disabled {
        background-color: #999;
        cursor: not-allowed;
    }

    .result-section {
        margin-top: 2rem;
        padding: 1rem;
        background-color: #f5f5f5;
        border-radius: 4px;
    }

    pre {
        font-size: 1rem;
        color: #333;
        white-space: pre-wrap;
    }
</style>
