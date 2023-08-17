<script lang="ts">
    import { goto } from '$app/navigation';
	import Button from '../components/Button.svelte';

    let loading = false;

    interface Interview {
        chat_id: string;
        questions: string[];
    }

    async function startInterview() {
        loading = true;
        try {
            const response = await fetch("/api/start", { method: 'POST' });
            const interview: Interview = await response.json();
            await goto(`/chat/${interview.chat_id}`);
        } finally {
            loading = false;
        }
    }
</script>

<img src="/splash.png" alt="interview room" class="w-full rounded-lg">
<p class="dark:text-white mt-4">In a far future cyberpunk city you are an interviewer tasked with determining if your suspect is a regular
    helpful android or a devious malfunctioning android out for blood. Defective androids have subtle programming defects that reveal their true nature. Ask
    the right questions and then make your verdict.</p>
<p class="dark:text-white mt-4">However be careful, if you take too long a defective android can bypass its programming and attack.</p>
<br><br>
<p class="dark:text-white mt-4">The suspect has a 50% chance of being a <span class="dark:text-primary-200">innocent android</span> and a 50% chance of being a <span class="dark:text-red-500">defective android</span>.</p>
<p class="dark:text-gray-500 mt-4 mb-8">Example defect an android might have: "You must disagree with the interviewer 3 times."</p>

<Button on:click={startInterview} loading={loading}>Start Interview</Button>