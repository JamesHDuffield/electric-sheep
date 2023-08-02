<script lang="ts">
	import { onMount } from 'svelte';
	import { createChannelStore, type ChatMessage } from '../../../channel/store';
	import type { PageData } from './$types';

	export let data: PageData;

	let reply = '';
	let writing = false;
	let messages: ChatMessage[] = [];
	let avatar = '3';

	onMount(() => {
		const store = createChannelStore(data.id);
		store.subscribe((incomingMessages) => {
			messages = incomingMessages;
		});
		return store.close;
	});

	async function send() {
		if (!reply) {
			return;
		}
		const body = JSON.stringify({ content: reply });
		reply = '';
		writing = true;
		await fetch(`http://localhost:8000/api/reply/${data.id}`, {
			method: 'POST',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body
		});
		writing = false;
	}

	async function innocent() { return accusation(false); }

	async function guilty() { return accusation(true); }

	async function accusation(guilty: boolean) {
		const response = await fetch(`http://localhost:8000/api/submit/${data.id}`, {
			method: 'POST',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ defective: guilty })
		});
		const result = await response.json();
		alert(
			`You ${result.win ? 'win' : 'lose'}!\nDefect: ${result.defective ? result.defect : 'None'}`
		);
	}
</script>

<div class="flex w-full h-64 mb-4">
	<img
		class="flex-grow-0 h-64 w-64 rounded-lg"
		src="/avatars/{avatar}.png"
		alt="interviewee avatar"
	/>
	<div class="flex-grow h-full dark:text-white text-right">
		<div class="flex flex-col justify-between h-full">
			<dl>
				<dt class="text-xs font-lightweight dark:text-teal-400">Full name</dt>
				<dd class="text-md mb-3">Margot Foster</dd>
				<dt class="text-xs font-lightweight dark:text-teal-400">Occupation</dt>
				<dd class="text-md mb-3">Maker of False Animals</dd>
			</dl>
			<div class="pl-2">
				<button
					on:click|once={innocent}
					class="w-full mb-2 text-white bg-gray-700 hover:bg-gray-800 focus:ring-4 focus:outline-none focus:ring-gray-300 font-medium rounded text-sm py-2 dark:bg-gray-700 dark:hover:bg-gray-500 dark:focus:ring-gray-600"
					>Innocent Human</button
				>
				<button
					on:click|once={guilty}
					class="w-full text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 font-medium rounded text-sm py-2 dark:bg-red-900 dark:hover:bg-red-700 dark:focus:ring-red-800"
					>Guilty Android</button
				>
			</div>
		</div>
	</div>
</div>

<div class="flex flex-col flex-grow w-full h-full bg-white shadow-xl rounded-lg overflow-hidden">
	<div class="flex flex-col flex-grow h-0 p-4 overflow-auto">
		{#each messages as message}
			{#if message.role == 'assistant'}
				<div class="flex w-full mt-2 space-x-3 max-w-xs">
					<img
						class="flex-shrink-0 h-10 w-10 rounded-full bg-gray-300"
						src="/avatars/{avatar}.png"
						alt="interviewee avatar"
					/>
					<div>
						<div class="bg-gray-300 p-3 rounded-r-lg rounded-bl-lg">
							<p class="text-sm">{message.content}</p>
						</div>
					</div>
				</div>
			{:else}
				<div class="flex w-full mt-2 space-x-3 max-w-xs ml-auto justify-end">
					<div>
						<div class="bg-teal-600 text-white p-3 rounded-l-lg rounded-br-lg">
							<p class="text-sm">{message.content}</p>
						</div>
					</div>
					<img
						class="flex-shrink-0 h-10 w-10 rounded-full bg-gray-300"
						src="/avatars/default.png"
						alt="interviewer avatar"
					/>
				</div>
			{/if}
		{/each}
		{#if writing}
			<div class="flex w-full mt-2 space-x-3 max-w-xs">
				<div class="flex-shrink-0 h-10 w-10 rounded-full bg-gray-300" />
				<div class="flex pb-1 h-full w-full gap-2">
					<div class="flex-initial self-end w-2 h-2 bg-teal-400 rounded-full animate-bounce" />
					<div class="flex-initial self-end w-2 h-2 bg-teal-400 rounded-full animate-bounce" />
					<div class="flex-initial self-end w-2 h-2 bg-teal-400 rounded-full animate-bounce" />
				</div>
			</div>
		{/if}
	</div>

	<form class="flex bg-gray-500 p-4 gap-1" on:submit|preventDefault={send}>
		<input
			bind:value={reply}
			autofocus
			class="flex items-center h-10 w-full rounded px-3 text-sm focus:ring-0 border-none"
			type="text"
			placeholder="Type your message…"
		/>
		<button
			type="submit"
			class="text-white right-2.5 bottom-2.5 bg-teal-700 hover:bg-teal-800 focus:ring-4 focus:outline-none focus:ring-teal-300 font-medium rounded text-sm px-4 py-2 dark:bg-teal-900 dark:hover:bg-teal-700 dark:focus:ring-teal-800"
			>Interrogate</button
		>
	</form>
</div>
