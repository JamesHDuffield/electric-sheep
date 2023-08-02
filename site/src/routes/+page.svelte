<script lang="ts">
    import { goto } from '$app/navigation';
    import { Button } from "flowbite-svelte";

    interface Interview {
        chat_id: string;
        questions: string[];
    }

    async function startInterview() {
        const response = await fetch("http://localhost:8000/api/start", { method: 'POST' });
        const interview: Interview = await response.json();
        await goto(`/chat/${interview.chat_id}`);
    }
</script>

<p class="dark:text-white">In a far future cyberpunk city you are an interviewer tasks with determining if your suspect is a regular
    human or a devious android. All androids have subtle programming defect that reveal their true nature. Ask
    the right questions and then make your verdict.</p><br><br>
<p class="dark:text-white mt-2">The suspect has a 50% chance of being a human and a 50% change of being an android.</p>
<p class="dark:text-white mt-2">Example defect an android might have: "You must always disagree with the interviewer."</p>
<Button on:click|once={startInterview} class="w-fit mt-4">
    Start Interview <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5 ml-2"><path stroke-linecap="round" stroke-linejoin="round" d="M13.5 4.5L21 12m0 0l-7.5 7.5M21 12H3" /></svg>
</Button>