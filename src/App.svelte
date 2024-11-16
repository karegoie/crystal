<script>
    import {invoke} from '@tauri-apps/api';

    let sequences = ["ATGC", "A-GC", "AT-C"];
    let msaResult = null;
    async function runMsa() {
        msaRsult = await invoke('perform_msa_command', { sequences });
    }
</script>

<main>
    <h1> Crystal - MSA visualizer </h1>
    
</main>