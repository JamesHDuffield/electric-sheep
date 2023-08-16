<script lang="ts">
	import { onMount } from 'svelte';
	import { createChannelStore, type ChatMessage } from '../../../channel/store';
	import type { PageData } from './$types';
	import { getChatDetails, type InterviewResult } from './service';
	import { goto } from '$app/navigation';

	export let data: PageData;

	let reply = '';
	let writing = false;
	let gameResult: InterviewResult | undefined = undefined;
	let messages: ChatMessage[] = [];
	let avatar = parseInt(data.id[0], 16) + 1; // Uses first letter of chat id to set avatar
	let status: 'Suspect' | 'Arresting Suspect' | 'Releasing Suspect' | 'Arrested' | 'Released' | 'Murderous' =
		'Suspect';

	$: interviewOver = status != 'Suspect';
	$: awaitingVerdict = status == 'Arresting Suspect' || status == 'Releasing Suspect';
	$: gameOver = status == 'Arrested' || status == 'Released' || status == 'Murderous' && gameResult;

	const details$ = getChatDetails(data.id);

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
		let response = await fetch(`/api/reply/${data.id}`, {
			method: 'POST',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body
		});
		let replyState: { result: InterviewResult | undefined } = await response.json();
		if (replyState.result) {
			status = 'Murderous';
			gameResult = replyState.result;
		}
		writing = false;
	}

	async function innocent() {
		return accusation(false);
	}

	async function guilty() {
		return accusation(true);
	}

	async function accusation(guilty: boolean) {
		status = guilty ? 'Arresting Suspect' : 'Releasing Suspect';
		const response = await fetch(`/api/submit/${data.id}`, {
			method: 'POST',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ defective: guilty })
		});
		gameResult = await response.json();

		// Wait for suspense...
		await new Promise((resolve) => setTimeout(resolve, 2000));

		status = guilty ? 'Arrested' : 'Released';
	}

	const scrollToBottom = (node: Element, _list: unknown[]) => {
		const scroll = () => node.scroll({
			top: node.scrollHeight,
			behavior: 'smooth',
		});
		scroll();
		return { update: scroll }
	};
</script>

<div class="flex w-full h-64 mb-4">
	<img
		class="flex-grow-0 w-64 max-w-[50%] object-cover rounded-lg"
		src="/avatars/{avatar}.png"
		alt="interviewee avatar"
	/>
	<div class="flex-grow h-full dark:text-white text-right">
		<div class="flex flex-col justify-between h-full">
			{#await details$}
				<p class="dark:text-white">...waiting</p>
			{:then details}
				<dl>
					<dt class="text-xs font-lightweight dark:text-teal-400">Full name</dt>
					<dd class="text-md mb-3">{details.name}</dd>
					<dt class="text-xs font-lightweight dark:text-teal-400">Occupation</dt>
					<dd class="text-md mb-3">{details.persona}</dd>
					<dt class="text-xs font-lightweight dark:text-teal-400">Status</dt>
					<dd class="text-md mb-3">
						{#if awaitingVerdict}
							<div
								class="inline-block h-4 w-4 animate-spin rounded-full border-4 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite]"
								role="status"
							>
								<span
									class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
									>...</span
								>
							</div>
						{/if}
						{status}
					</dd>
				</dl>
			{:catch error}
				<p class="dark:text-red">Failed To Load Chat Details: {error.message}</p>
			{/await}
			<div
				class="pl-2 opacity-1 transition-opacity ease-linear delay-150"
				class:opacity-0={interviewOver}
			>
				<button
					on:click|once={innocent}
					disabled={interviewOver}
					class="w-full mb-2 text-white bg-gray-700 hover:bg-gray-800 focus:ring-4 focus:outline-none focus:ring-gray-300 font-medium rounded text-sm py-2 dark:bg-gray-700 dark:hover:bg-gray-500 dark:focus:ring-gray-600"
					>Innocent Android</button
				>
				<button
					on:click|once={guilty}
					disabled={interviewOver}
					class="w-full text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 font-medium rounded text-sm py-2 dark:bg-red-900 dark:hover:bg-red-700 dark:focus:ring-red-800"
					>Defective Android</button
				>
			</div>
		</div>
	</div>
</div>

<div class="flex flex-col flex-grow w-full h-full bg-white shadow-xl rounded-lg overflow-hidden mb-3">
	<div use:scrollToBottom={messages} class="flex flex-col flex-grow h-0 p-4 overflow-auto">
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
	<form
		class="flex bg-gray-500 p-4 gap-1 opacity-1 transition-opacity ease-linear delay-150"
		on:submit|preventDefault={send}
		class:opacity-0={interviewOver}
	>
		<!-- svelte-ignore a11y-autofocus -->
		<input
			bind:value={reply}
			disabled={interviewOver}
			autofocus
			class="flex items-center h-10 w-full rounded px-3 text-sm focus:ring-0 border-none"
			type="text"
			placeholder="Type your message…"
		/>
		<button
			type="submit"
			disabled={interviewOver}
			class="text-white right-2.5 bottom-2.5 bg-teal-700 hover:bg-teal-800 focus:ring-4 focus:outline-none focus:ring-teal-300 font-medium rounded text-sm px-4 py-2 dark:bg-teal-900 dark:hover:bg-teal-700 dark:focus:ring-teal-800"
			>➤</button
		>
	</form>
</div>

<div class="fixed left-0 top-0 z-[1055] h-full w-full overflow-y-auto overflow-x-hidden outline-none bg-gray-800/50 transition-opacity ease-linear-in delay-500 pointer-events-none" class:opacity-0={!gameOver} class:opacity-1={gameOver}>
	<div class="pointer-events-none relative flex min-h-[calc(100%-1rem)] w-auto translate-y-[-50px] items-center min-[576px]:mx-auto min-[576px]:mt-7 min-[576px]:min-h-[calc(100%-3.5rem)] min-[576px]:max-w-[500px]">
		<div class="pointer-events-auto relative flex w-full flex-col rounded-md border-none bg-white bg-clip-padding text-current shadow-lg outline-none dark:dark:bg-slate-950">
			<div class="flex flex-col items-center justify-between rounded-t-md p-4 gap-3">
				{#if gameResult}
					{#if gameResult.defective}
						{#if gameResult.win}
							<h3 class="dark:text-teal-200">Successful Arrest</h3>
							<p class="dark:text-white text-sm text-center">Another dangerous android off the streets.</p>
						{:else}
							{#if status == 'Murderous'}
								<h3 class="dark:text-red-500">Android Attack</h3>
								<p class="dark:text-white text-sm text-center">The android leaps over the table and swiftly executes you.</p>
							{:else}
								<h3 class="dark:text-red-500">Wrongful Release</h3>
								<p class="dark:text-white text-sm text-center">The android leaves promptly. You are notified via hologram of its crimes later that week; along with your termination from the company.</p>
							{/if}
						{/if}
						<p class="dark:text-gray-500 text-xs text-center">The android had the following defect: "{gameResult.defect}"</p>
					{:else}
						{#if gameResult.win}
							<h3 class="dark:text-teal-200">Successful Release</h3>
							<p class="dark:text-white text-sm text-center">The suspect thanks you for your service.</p>
						{:else}
							<h3 class="dark:text-red-500">Wrongful Arrest</h3>
							<p class="dark:text-white text-sm text-center">The suspect cries out as they are dragged away by security.</p>
						{/if}
						<p class="dark:text-gray-500 text-xs text-center">The android was functioning without defect.</p>
					{/if}
				{/if}
				<a href="/"><button class="text-white bg-teal-700 hover:bg-teal-800 focus:ring-4 focus:outline-none focus:ring-teal-300 font-medium rounded text-sm px-4 py-2 dark:bg-teal-900 dark:hover:bg-teal-700 dark:focus:ring-teal-800">Quit</button></a>
			</div>
		</div>
	</div>
</div>

